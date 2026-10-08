import {
  createClient,
  generateKeyPairSigner,
  lamports,
  type KeyPairSigner,
} from "@solana/kit";
import { solanaLocalRpc } from "@solana/kit-plugin-rpc";
import { airdropSigner, signer } from "@solana/kit-plugin-signer";
import { freelanc3Program } from "../../clients/typescript/src/generated";

export async function setupClient() {
  const payer = await generateKeyPairSigner();
  const client = await createClient()
    .use(signer(payer))
    .use(solanaLocalRpc())
    .use(airdropSigner(lamports(10_000_000_000n)))
    .use(freelanc3Program());
  return client;
}

export async function createTestProfile(
  client: Awaited<ReturnType<typeof setupClient>>,
  username: string = "Shivam",
  authority: KeyPairSigner = client.payer,
) {
  const [profilePDA] = await client.freelanc3.pdas.profile({
    authority: authority.address,
  });
  await client.freelanc3.instructions
    .createProfile({ authority, username })
    .sendTransaction();
  return profilePDA;
}

export async function createTestJob(
  client: Awaited<ReturnType<typeof setupClient>>,
  jobId: bigint = 1n,
  title: string = "Solana Developer Relation",
  budget: number = 100_000,
  poster: KeyPairSigner = client.payer,
) {
  const [jobPDA] = await client.freelanc3.pdas.job({
    poster: poster.address,
    jobId,
  });
  await client.freelanc3.instructions
    .createJob({ poster, jobId, title, budget })
    .sendTransaction();
  return jobPDA;
}
