import { describe, it, before } from "mocha";
import { expect } from "chai";
import {
  generateKeyPairSigner,
  lamports,
  some,
  type Address,
  type KeyPairSigner,
} from "@solana/kit";
import {
  Status,
  ApplicationtStatus,
} from "../../clients/typescript/src/generated";
import { setupClient, createTestProfile, createTestJob } from "./helper";

describe("freelanc3", function () {
  this.timeout(60_000);

  let client: Awaited<ReturnType<typeof setupClient>>;
  let employerProfilePDA: Awaited<ReturnType<typeof createTestProfile>>;
  let applicantProfilePDA: Awaited<ReturnType<typeof createTestProfile>>;
  let jobPDA: Awaited<ReturnType<typeof createTestJob>>;
  let applicationPDA: Address;
  let applicant: KeyPairSigner;

  before(async () => {
    client = await setupClient();

    applicant = await generateKeyPairSigner();
    await client.airdrop(applicant.address, lamports(5_000_000_000n));

    employerProfilePDA = await createTestProfile(client, "Shivam");
    applicantProfilePDA = await createTestProfile(client, "Deepesh", applicant);
    jobPDA = await createTestJob(client, 1n, "Solana Developer Relation", 100_000);

    [applicationPDA] = await client.freelanc3.pdas.application({
      job: jobPDA,
      applicant: applicant.address,
    });
  });

  it("creates a profile of employer and applicant", async () => {
    const employer = await client.freelanc3.accounts.profile.fetch(
      employerProfilePDA,
    );
    expect(employer.data.username).to.equal("Shivam");
    expect(employer.data.authority).to.equal(client.payer.address);

    const applicantProfile = await client.freelanc3.accounts.profile.fetch(
      applicantProfilePDA,
    );
    expect(applicantProfile.data.username).to.equal("Deepesh");
    expect(applicantProfile.data.authority).to.equal(applicant.address);
  });

  it("creates a job", async () => {
    const job = await client.freelanc3.accounts.job.fetch(jobPDA);
    expect(job.data.poster).to.equal(client.payer.address);
    expect(job.data.jobId).to.equal(1n);
    expect(job.data.budget).to.equal(100_000);
    expect(job.data.status).to.equal(Status.Opened);
  });

  it("applies for a job", async () => {
    await client.freelanc3.instructions
      .applyJob({ applicant, job: jobPDA, resumeRef: "resume-cid" })
      .sendTransaction();

    const application = await client.freelanc3.accounts.application.fetch(
      applicationPDA,
    );
    expect(application.data.job).to.equal(jobPDA);
    expect(application.data.applicant).to.equal(applicant.address);
    expect(application.data.status).to.equal(ApplicationtStatus.Applied);
  });

  it("non-poster cannot hire", async () => {
    let failed = false;
    try {
      await client.freelanc3.instructions
        .hire({ poster: applicant, job: jobPDA, application: applicationPDA })
        .sendTransaction();
    } catch {
      failed = true;
    }
    expect(failed).to.equal(true);
  });

  it("poster hires the applicant", async () => {
    await client.freelanc3.instructions
      .hire({ poster: client.payer, job: jobPDA, application: applicationPDA })
      .sendTransaction();

    const job = await client.freelanc3.accounts.job.fetch(jobPDA);
    expect(job.data.status).to.equal(Status.Hired);
    expect(job.data.hiredApplicant).to.deep.equal(some(applicant.address));

    const application = await client.freelanc3.accounts.application.fetch(
      applicationPDA,
    );
    expect(application.data.status).to.equal(ApplicationtStatus.Accepted);
  });
});
