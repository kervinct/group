import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { UserGroup } from '../target/types/user_group';
import * as spl from "@solana/spl-token";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { u32 } from 'buffer-layout';
import { assert } from 'chai';

const print = console.log;
anchor.setProvider(anchor.Provider.env());
const provider = anchor.getProvider();
const program = anchor.workspace.UserGroup as Program<UserGroup>;

type Publickey = anchor.web3.PublicKey;
type Keypair = anchor.web3.Keypair;
const LAMPORTS_PER_SOL = anchor.web3.LAMPORTS_PER_SOL;

describe('user-group', () => {
  // Configure the client to use the local cluster.
  const payer: Keypair = anchor.web3.Keypair.generate();
  const adminKeypair: Keypair = anchor.web3.Keypair.generate();
  const burnTokenMintKeypair: Keypair = anchor.web3.Keypair.generate();
  const userOneKeypair: Keypair = anchor.web3.Keypair.generate();
  const userTwoKeypair: Keypair = anchor.web3.Keypair.generate();
  const userThreeKeypair: Keypair = anchor.web3.Keypair.generate();
  const decimals = 6;

  let userOneBurnToken: Publickey;
  let userTwoBurnToken: Publickey;
  let userThreeBurnToken: Publickey;
  
  let admin: Publickey;
  let group: Publickey;
  let groupBurnToken: Publickey;
  // let proposal: Publickey;
  // let signature: Publickey;
  let memberOne: Publickey;
  let memberTwo: Publickey;
  let memberThree: Publickey;

  it('prepare accounts', async () => {
    print(`payer Pubkey: ${payer.publicKey.toBase58()}`);
    print(`admin Pubkey: ${adminKeypair.publicKey.toBase58()}`);
    print(`userOne Pubkey: ${userOneKeypair.publicKey.toBase58()}`);
    print(`userTwo Pubkey: ${userTwoKeypair.publicKey.toBase58()}`);
    print(`userThree Pubkey: ${userThreeKeypair.publicKey.toBase58()}`);

    await provider.connection.requestAirdrop(
      payer.publicKey,
      1000 * LAMPORTS_PER_SOL,
    );

    await provider.connection.requestAirdrop(
      adminKeypair.publicKey,
      1000 * LAMPORTS_PER_SOL,
    );

    await provider.connection.requestAirdrop(
      userOneKeypair.publicKey,
      10 * LAMPORTS_PER_SOL,
    );

    await provider.connection.requestAirdrop(
      userTwoKeypair.publicKey,
      10 * LAMPORTS_PER_SOL,
    );

    await provider.connection.requestAirdrop(
      userThreeKeypair.publicKey,
      10 * LAMPORTS_PER_SOL,
    );

    await spl.createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      decimals,
      burnTokenMintKeypair,
    );
    print(`mint Pubkey: ${burnTokenMintKeypair.publicKey.toBase58()}`)

    userOneBurnToken = await spl.createAssociatedTokenAccount(
      provider.connection,
      userOneKeypair,
      burnTokenMintKeypair.publicKey,
      userOneKeypair.publicKey,
    );
    print(`userOneBurnToken Pubkey: ${userOneBurnToken.toBase58()}`);

    userTwoBurnToken = await spl.createAssociatedTokenAccount(
      provider.connection,
      userTwoKeypair,
      burnTokenMintKeypair.publicKey,
      userTwoKeypair.publicKey,
    );
    print(`userTwoBurnToken Pubkey: ${userTwoBurnToken.toBase58()}`);

    userThreeBurnToken = await spl.createAssociatedTokenAccount(
      provider.connection,
      userThreeKeypair,
      burnTokenMintKeypair.publicKey,
      userThreeKeypair.publicKey,
    );
    print(`userThreeBurnToken Pubkey: ${userThreeBurnToken.toBase58()}`);

    await spl.mintTo(
      provider.connection,
      payer,
      burnTokenMintKeypair.publicKey,
      userOneBurnToken,
      payer,
      1000 * (10 ** decimals),
    );
    await spl.mintTo(
      provider.connection,
      payer,
      burnTokenMintKeypair.publicKey,
      userTwoBurnToken,
      payer,
      1000 * (10 ** decimals),
    );
    await spl.mintTo(
      provider.connection,
      payer,
      burnTokenMintKeypair.publicKey,
      userThreeBurnToken,
      payer,
      1000 * (10 ** decimals),
    );

    print("complete prepare accounts");
  });

  it("initialize admin", async() => {
    const [adminPubkey, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [payer.publicKey.toBuffer(), Buffer.from("group_admin")],
      program.programId,
    );
    admin = adminPubkey;
    print(`admin Pubkey: ${admin.toBase58()}`);

    const tx = await program.rpc.initialize(
      nonce,
      {
        accounts: {
          authority: payer.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          admin: admin,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [payer],
      }
    );
    print(`Initialized the admin transaction: ${tx}`);

    const adminAccount = await program.account.adminAccount.fetch(admin);
    assert.equal(adminAccount.seed.toString(), nonce.toString());
    assert.equal(adminAccount.current.toString(), "1");
    assert.equal(adminAccount.initialized, true);
    assert.equal(adminAccount.tokenMint.toBase58(), burnTokenMintKeypair.publicKey.toBase58());
    print(`now have admins: `);
    for (let i =0; i< adminAccount.current; i++) {
      print(`   ${adminAccount.administrators[i].toBase58()}`);
    }

    print("complete initialize admin");
  });

  it("create group for userOne", async() => {
    const [groupPubkey, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [admin.toBuffer(), Buffer.from([0, 0, 0, 0]), Buffer.from("group")],
      program.programId,
    );
    group = groupPubkey;
    print(`group Pubkey: ${group.toBase58()}`);

    const token = await spl.getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      burnTokenMintKeypair.publicKey,
      group,
      true,
    );
    groupBurnToken = token.address;
    print(`groupBurnToken Pubkey: ${groupBurnToken.toBase58()}`);

    const tx = await program.rpc.createGroup(
      nonce,
      10,
      {
        accounts: {
          authority: payer.publicKey,
          sponsor: userOneKeypair.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          group: group,
          token: groupBurnToken,
          admin: admin,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [payer],
      }
    );
    print(`Created the group transaction: ${tx}`);

    const groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.seed.toString(), nonce.toString());
    assert.isFalse(groupAccount.electing);
    assert.isFalse(groupAccount.freeze);
    assert.equal(groupAccount.rate.numerator.toString(), "100");
    assert.equal(groupAccount.rate.denominator.toString(), "100");
    assert.equal(groupAccount.maxManager.toString(), "10");
    assert.equal(groupAccount.currentManager.toString(), "0");
    assert.equal(groupAccount.currentMember.toString(), "0");
    assert.equal(groupAccount.proposals.toString(), "0");
    assert.equal(groupAccount.index.toString(), "0");
    assert.equal(groupAccount.sponsor.toBase58(), userOneKeypair.publicKey.toBase58());
    assert.equal(groupAccount.admin.toBase58(), admin.toBase58());

    const [member] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), userOneKeypair.publicKey.toBuffer(), Buffer.from("member")],
      program.programId,
    );
    memberOne = member;
    print(`memberOne Pubkey: ${memberOne.toBase58()}`);

    const tx2 = await program.rpc.joinGroup(
      {
        accounts: {
          authority: payer.publicKey,
          user: userOneKeypair.publicKey,
          group: group,
          member: memberOne,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [payer],
      }
    );
    print(`Joined the group transaction: ${tx2}`);

    const memberAccount = await program.account.memberAccount.fetch(memberOne);
    assert.equal(JSON.stringify(memberAccount.position), JSON.stringify({ manager : {} }));
    assert.isFalse(memberAccount.inPromotion);
    assert.isFalse(memberAccount.inWithdraw);
    assert.equal(memberAccount.group.toBase58(), group.toBase58());
    assert.equal(memberAccount.funder.toBase58(), payer.publicKey.toBase58());
    assert.equal(memberAccount.owner.toBase58(), userOneKeypair.publicKey.toBase58());

    print("complete create group for userOne");
  });

  it("add admin and remove admin", async () => {
    const tx = await program.rpc.addAdmin(
      {
        accounts: {
          authority: payer.publicKey,
          user: adminKeypair.publicKey,
          admin,
        },
        signers: [payer],
      }
    );
    print(`added admin transaction: ${tx}`);

    let adminAccount = await program.account.adminAccount.fetch(admin);
    
    assert.equal(adminAccount.current.toString(), "2");
    print(`admins: `);
    for (let i =0; i< adminAccount.current; i++) {
      print(`    ${adminAccount.administrators[i].toBase58()}`);
    }

    const tx2 = await program.rpc.removeAdmin(
      {
        accounts: {
          authority: adminKeypair.publicKey,
          user: payer.publicKey,
          admin,
        },
        signers: [adminKeypair],
      }
    );
    print(`removed admin transaction: ${tx2}`);

    adminAccount = await program.account.adminAccount.fetch(admin);
    assert.equal(adminAccount.current.toString(), "1");
    print(`admins: `);
    for (let i =0; i< adminAccount.current; i++) {
      print(`    ${adminAccount.administrators[i].toBase58()}`);
    }

    print("complete add admin and remove admin");
  });

  it("freeze and thaw group", async () => {
    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.isFalse(groupAccount.freeze);

    const tx = await program.rpc.freezeGroup(
      {
        accounts: {
          authority: adminKeypair.publicKey,
          admin,
          group,
        },
        signers: [adminKeypair],
      }
    );
    print(`Freeze group transaction: ${tx}`);

    groupAccount = await program.account.groupAccount.fetch(group);
    assert.isTrue(groupAccount.freeze);

    const tx2 = await program.rpc.thawGroup(
      {
        accounts: {
          authority: adminKeypair.publicKey,
          admin,
          group,
        },
        signers: [adminKeypair],
      }
    );
    print(`Thawed group transaction: ${tx2}`);

    groupAccount = await program.account.groupAccount.fetch(group);
    assert.isFalse(groupAccount.freeze);

    print("complete freeze and thaw group");
  });

  it("join and exit group", async () => {
    const [member2, nonce2] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), userTwoKeypair.publicKey.toBuffer(), Buffer.from("member")],
      program.programId,
    );
    memberTwo = member2;
    print(`memberTwo Pubkey: ${memberTwo.toBase58()}`);

    const tx = await program.rpc.joinGroup(
      {
        accounts: {
          authority: userTwoKeypair.publicKey,
          user: userTwoKeypair.publicKey,
          group,
          member: memberTwo,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userTwoKeypair],
      }
    );
    print(`Joined group transaction: ${tx}`);

    const memberAccount = await program.account.memberAccount.fetch(memberTwo);
    assert.equal(JSON.stringify(memberAccount.position), JSON.stringify({ member : {} }));
    assert.isFalse(memberAccount.inPromotion);
    assert.isFalse(memberAccount.inWithdraw);
    assert.equal(memberAccount.group.toBase58(), group.toBase58());
    assert.equal(memberAccount.funder.toBase58(), userTwoKeypair.publicKey.toBase58());
    assert.equal(memberAccount.owner.toBase58(), userTwoKeypair.publicKey.toBase58());

    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.currentManager.toString(), "1");
    assert.equal(groupAccount.currentMember.toString(), "1");

    const tx2 = await program.rpc.exitGroup(
      {
        accounts: {
          authority: userTwoKeypair.publicKey,
          funder: userTwoKeypair.publicKey,
          member: memberTwo,
          group,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userTwoKeypair],
      }
    );
    print(`Exited group transaction: ${tx2}`);

    const info = await provider.connection.getAccountInfo(memberTwo);
    assert.isNull(info);

    groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.currentManager.toString(), "1");
    assert.equal(groupAccount.currentMember.toString(), "0");

    print("complete join and exit group");
  });

  it("join group", async () => {
    const tx = await program.rpc.joinGroup(
      {
        accounts: {
          authority: userTwoKeypair.publicKey,
          user: userTwoKeypair.publicKey,
          group,
          member: memberTwo,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userTwoKeypair],
      }
    );
    print(`Joined group transaction: ${tx}`);

    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.currentManager.toString(), "1");
    assert.equal(groupAccount.currentMember.toString(), "1");

    const [member3, nonce3] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), userThreeKeypair.publicKey.toBuffer(), Buffer.from("member")],
      program.programId,
    );
    memberThree = member3;
    print(`memberThree Pubkey: ${memberThree.toBase58()}`);

    const tx2 = await program.rpc.joinGroup(
      {
        accounts: {
          authority: userThreeKeypair.publicKey,
          user: userThreeKeypair.publicKey,
          group,
          member: memberThree,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userThreeKeypair],
      }
    );
    print(`Joined group transaction: ${tx2}`);

    groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.currentManager.toString(), "1");
    assert.equal(groupAccount.currentMember.toString(), "2");

    print("complete join group");
  });

  it("revoke the proposal", async () => {
    // since user one is admin
    let groupAccount = await program.account.groupAccount.fetch(group);
    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );
    const deadline = new Date().getTime() + 43200 * 1000;
    const tx = await program.rpc.submitProposal(
      {upgrade: {}},
      new anchor.BN(10 * (10 ** decimals)),
      new anchor.BN(deadline / 1000),
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          beneficiary: userTwoKeypair.publicKey,
          proposal,
          group,
          member: memberOne,
          beneMember: memberTwo,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userOneKeypair],
      }
    );
    print(`Submit proposal transaction: ${tx}`);
    let memberAccount = await program.account.memberAccount.fetch(memberTwo);
    assert.isTrue(memberAccount.inPromotion);
    
    const tx2 = await program.rpc.revokeProposal(
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          beneficiary: userTwoKeypair.publicKey,
          beneMember: memberTwo,
          proposal,
          group,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userOneKeypair],
      }
    );
    print(`Revoke proposal transaction: ${tx2}`);

    memberAccount = await program.account.memberAccount.fetch(memberTwo);
    assert.isFalse(memberAccount.inPromotion);
  })

  it("upgrade group rate", async () => {
    const rate = new participateRate();
    rate.numerator = new anchor.BN(20);
    rate.denominator = new anchor.BN(100);

    const tx = await program.rpc.upgradeGroup(
      rate,
      {
        accounts: {
          authority: adminKeypair.publicKey,
          admin,
          group,
        },
        signers: [adminKeypair],
      }
    );
    print(`Upgraded group transaction: ${tx}`);

    const groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.rate.numerator.toString(), "20");
    assert.equal(groupAccount.rate.denominator.toString(), "100");

    print("complete upgrade group rate");
  });

  it("submit upgrade proposal and agreed", async() => {
    const groupAccount = await program.account.groupAccount.fetch(group);
    assert.strictEqual(groupAccount.proposals, 1);

    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );

    const deadline = new Date().getTime() + 43200 * 1000;
    const tx = await program.rpc.submitProposal(
      {upgrade: {}},
      new anchor.BN(10 * (10 ** decimals)),
      new anchor.BN(deadline / 1000),
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          beneficiary: userTwoKeypair.publicKey,
          proposal,
          group,
          member: memberOne,
          beneMember: memberTwo,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userOneKeypair],
      }
    );
    print(`submit proposal transaction: ${tx}`);

    // await showProposal(proposal);
    let proposalAccount = await program.account.proposalAccount.fetch(proposal);
    assert.equal(proposalAccount.submitter.toBase58(), userOneKeypair.publicKey.toBase58());
    assert.equal(proposalAccount.beneficiary.toBase58(), userTwoKeypair.publicKey.toBase58());
    assert.equal(proposalAccount.beneMember.toBase58(), memberTwo.toBase58());
    assert.equal(proposalAccount.group.toBase58(), group.toBase58());
    assert.equal(proposalAccount.positive.toString(), "0");
    assert.equal(proposalAccount.negative.toString(), "0");
    assert.equal(proposalAccount.limit.toString(), "10000000");
    assert.equal(JSON.stringify(proposalAccount.proposalType), JSON.stringify({upgrade:{}}));
    assert.equal(JSON.stringify(proposalAccount.status), JSON.stringify({progressing: {}}));

    print(`deadline: ${new Date(parseInt(proposalAccount.deadline.toString()) * 1000)}`)
    print(`revoke_timeout: ${new Date(parseInt(proposalAccount.revokeTimeout.toString()) * 1000)}`);

    let memberAccount = await program.account.memberAccount.fetch(memberTwo);
    assert.isTrue(memberAccount.inPromotion);

    const [signature, n] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), memberOne.toBuffer(), proposal.toBuffer()],
      program.programId,
    );
    print(`user one signature Pubkey: ${signature.toBase58()}`);

    const tx2 = await program.rpc.signProposal(
      {agreed: {}},
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          group,
          token: userOneBurnToken,
          member: memberOne,
          signature,
          proposal,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        remainingAccounts: [
          {pubkey: memberTwo, isSigner: false, isWritable: true},
        ],
        signers: [userOneKeypair],
      }
    );
    print(`user one sign proposal transaction: ${tx2}`);

    // now 20 percent hence, so passed immediately
    proposalAccount = await program.account.proposalAccount.fetch(proposal);
    assert.equal(proposalAccount.positive.toString(), "1");
    assert.equal(proposalAccount.negative.toString(), "0");
    print(`proposal status: ${JSON.stringify(proposalAccount.status)}`);

    memberAccount = await program.account.memberAccount.fetch(memberTwo);
    assert.isFalse(memberAccount.inPromotion);
    assert.equal(JSON.stringify(memberAccount.position), JSON.stringify({manager: {}}));

    const tx4 = await program.rpc.closeProposal(
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          proposal,
          group,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userOneKeypair],
      }
    );
    print(`close proposal transaction: ${tx4}`);

    const proposalAccountInfo = await provider.connection.getAccountInfo(proposal);
    assert.isNull(proposalAccountInfo);

    const tx3 = await program.rpc.closeSignature(
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          group,
          member: memberOne,
          proposal,
          signature,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userOneKeypair],
      }
    );
    print(`close signature transaction: ${tx3}`);

    print("complete submit upgrade proposal");
  });

  it("user three deposit token", async () => {
    let groupToken = await spl.getAccount(provider.connection, groupBurnToken);
    assert.equal(groupToken.amount.toString(), "0");

    const tx = await program.rpc.depositToken(
      new anchor.BN(500 * (10 ** decimals)),
      {
        accounts: {
          authority: userThreeKeypair.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          member: memberThree,
          token: userThreeBurnToken,
          group,
          vault: groupBurnToken,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [userThreeKeypair],
      }
    );
    print(`deposit token transaction: ${tx}`);

    groupToken = await spl.getAccount(provider.connection, groupBurnToken);
    assert.equal(groupToken.amount.toString(), "500000000");

    print("complete deposit token");
  });

  it("submit downgrade proposal and denied", async () => {
    const groupAccount = await program.account.groupAccount.fetch(group);
    assert.strictEqual(groupAccount.proposals, 2);
    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );

    const deadline = new Date().getTime() + 43200 * 1000;
    const tx = await program.rpc.submitProposal(
      {downgrade: {}},
      new anchor.BN(10 * (10 ** decimals)),
      new anchor.BN(deadline / 1000),
      {
        accounts: {
          authority: userTwoKeypair.publicKey,
          beneficiary: userOneKeypair.publicKey,
          proposal,
          group,
          member: memberTwo,
          beneMember: memberOne,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userTwoKeypair],
      }
    );
    print(`submit proposal transaction: ${tx}`);

    let proposalAccount = await program.account.proposalAccount.fetch(proposal);
    assert.strictEqual(proposalAccount.positive, 0);
    assert.strictEqual(proposalAccount.negative, 0);

    const [signature, n] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), memberOne.toBuffer(), proposal.toBuffer()],
      program.programId,
    );
    print(`user one signature Pubkey: ${signature.toBase58()}`);

    const tx2 = await program.rpc.signProposal(
      {denied: {}},
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          group,
          token: userOneBurnToken,
          member: memberOne,
          signature,
          proposal,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        remainingAccounts: [
          {pubkey: memberOne, isSigner: false, isWritable: true},
        ],
        signers: [userOneKeypair],
      }
    );
    print(`user one sign proposal transaction: ${tx2}`);

    // now 20 percent hence, so rejected immediately
    proposalAccount = await program.account.proposalAccount.fetch(proposal);
    assert.strictEqual(proposalAccount.positive, 0);
    assert.strictEqual(proposalAccount.negative, 1);
    print(`proposal status: ${JSON.stringify(proposalAccount.status)}`);

    // left signature and proposal remain, do not close

    print("complete submit downgrade proposal");
  });

  it("user one submit proposal update group", async () => {
    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.strictEqual(groupAccount.proposals, 3);
    assert.strictEqual(groupAccount.maxManager, 10);
    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );

    const deadline = new Date().getTime() + 43200 * 1000;

    try {
      await program.rpc.submitProposal(
        {updateGroup: {maxManager: new anchor.BN(20)}},
        new anchor.BN(10 * (10 ** decimals)),
        new anchor.BN(deadline / 1000),
        {
          accounts: {
            authority: userOneKeypair.publicKey,
            beneficiary: userOneKeypair.publicKey,
            proposal,
            group,
            member: memberOne,
            beneMember: memberOne,
            systemProgram: anchor.web3.SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          },
          signers: [userOneKeypair],
        }
      );
    } catch (err) {
      assert.strictEqual(err.msg, "Could not update the group");
      assert.strictEqual(err.code, 6013);
    }

    print("complete submit proposal update group");
  });

  it("user three submit proposal upgrade", async () => {
    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.strictEqual(groupAccount.proposals, 3);
    assert.strictEqual(groupAccount.maxManager, 10);
    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );

    const deadline = new Date().getTime() + 43200 * 1000;

    try {
      await program.rpc.submitProposal(
        {upgrade: {}},
        new anchor.BN(10 * (10 ** decimals)),
        new anchor.BN(deadline / 1000),
        {
          accounts: {
            authority: userThreeKeypair.publicKey,
            beneficiary: userThreeKeypair.publicKey,
            proposal,
            group,
            member: memberThree,
            beneMember: memberThree,
            systemProgram: anchor.web3.SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
          },
          signers: [userThreeKeypair],
        }
      );
    } catch (err) {
      assert.strictEqual(err.msg, "No permission to submit such proposal");
      assert.strictEqual(err.code, 6006);
    }
  });

  it("user one submit proposal withdraw", async () => {
    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.strictEqual(groupAccount.proposals, 3);
    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );
    const deadline = new Date().getTime() + 43200 * 1000;

    let groupTokenAccount = await spl.getAccount(provider.connection, groupBurnToken);
    assert.equal(groupTokenAccount.amount.toString(), "500000000");
    let userOneTokenAccount = await spl.getAccount(provider.connection, userOneBurnToken);
    print(`user one token balance: ${userOneTokenAccount.amount.toString()}`);

    const tx = await program.rpc.submitProposal(
      {withdraw: {mint: burnTokenMintKeypair.publicKey, receiver: userOneBurnToken, amount: new anchor.BN(100 * (10 ** decimals))}},
      new anchor.BN(10 * (10 ** decimals)),
      new anchor.BN(deadline / 1000),
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          beneficiary: userOneKeypair.publicKey,
          proposal,
          group,
          member: memberOne,
          beneMember: memberOne,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        remainingAccounts: [
          {pubkey: groupBurnToken, isSigner: false, isWritable: true},
        ],
        signers: [userOneKeypair],
      }
    );
    print(`user one sign proposal transaction: ${tx}`);

    const [signature, n] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), memberTwo.toBuffer(), proposal.toBuffer()],
      program.programId,
    );
    print(`user two signature Pubkey: ${signature.toBase58()}`);

    let memberOneAccount = await program.account.memberAccount.fetch(memberOne);
    assert.isTrue(memberOneAccount.inWithdraw);

    const tx2 = await program.rpc.signProposal(
      {agreed: {}},
      {
        accounts: {
          authority: userTwoKeypair.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          group,
          token: userTwoBurnToken,
          member: memberTwo,
          signature,
          proposal,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        remainingAccounts: [
          {pubkey: memberOne, isSigner: false, isWritable: true},
          {pubkey: groupBurnToken, isSigner: false, isWritable: true},
          {pubkey: userOneBurnToken, isSigner: false, isWritable: true},
        ],
        signers: [userTwoKeypair],
      }
    );
    print(`user two sign proposal transaction: ${tx2}`);

    groupTokenAccount = await spl.getAccount(provider.connection, groupBurnToken);
    assert.equal(groupTokenAccount.amount.toString(), "400000000");

    memberOneAccount = await program.account.memberAccount.fetch(memberOne);
    assert.isFalse(memberOneAccount.inWithdraw);

    userOneTokenAccount = await spl.getAccount(provider.connection, userOneBurnToken);
    print(`user one token balance: ${userOneTokenAccount.amount.toString()}`);

    print("complete submit proposal withdraw");
  });

  it("user three submit proposal reelection", async () => {
    const tx = await program.rpc.exitGroup(
      {
        accounts: {
          authority: userOneKeypair.publicKey,
          funder: payer.publicKey,
          member: memberOne,
          group,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userOneKeypair],
      }
    );
    print(`user one exit group transaction: ${tx}`);

    const tx2 = await program.rpc.exitGroup(
      {
        accounts: {
          authority: userTwoKeypair.publicKey,
          funder: userTwoKeypair.publicKey,
          member: memberTwo,
          group,
          systemProgram: anchor.web3.SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userTwoKeypair],
      }
    );
    print(`user two exit group transaction: ${tx2}`);

    let groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.currentManager.toString(), "0");
    assert.equal(groupAccount.currentMember.toString(), "1");
    assert.strictEqual(groupAccount.proposals, 4);
    assert.equal(groupAccount.maxManager.toString(), "10");
    assert.isFalse(groupAccount.electing);

    let memberThreeAccount = await program.account.memberAccount.fetch(memberThree);
    assert.isFalse(memberThreeAccount.inPromotion);

    const buf = Buffer.alloc(4);
    u32().encode(groupAccount.proposals, buf);
    const [proposal, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), buf, Buffer.from("proposal")],
      program.programId,
    );
    const deadline = new Date().getTime() + 43200 * 1000;
    const tx3 = await program.rpc.submitProposal(
      {reElection: {}},
      new anchor.BN(10 * (10 ** decimals)),
      new anchor.BN(deadline / 1000),
      {
        accounts: {
          authority: userThreeKeypair.publicKey,
          beneficiary: userThreeKeypair.publicKey,
          proposal,
          group,
          member: memberThree,
          beneMember: memberThree,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        signers: [userThreeKeypair],
      }
    );
    print(`user three submit proposal transaction: ${tx3}`);

    groupAccount = await program.account.groupAccount.fetch(group);
    assert.isTrue(groupAccount.electing);

    memberThreeAccount = await program.account.memberAccount.fetch(memberThree);
    // console.log(memberThreeAccount.inPromotion);
    assert.isTrue(memberThreeAccount.inPromotion);

    const [signature, n] = await anchor.web3.PublicKey.findProgramAddress(
      [group.toBuffer(), memberThree.toBuffer(), proposal.toBuffer()],
      program.programId,
    );
    print(`user three signature Pubkey: ${signature.toBase58()}`);

    const tx4 = await program.rpc.signProposal(
      {agreed: {}},
      {
        accounts: {
          authority: userThreeKeypair.publicKey,
          mint: burnTokenMintKeypair.publicKey,
          group,
          token: userThreeBurnToken,
          member: memberThree,
          signature,
          proposal,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        },
        remainingAccounts: [
          {pubkey: memberThree, isSigner: false, isWritable: true},
        ],
        signers: [userThreeKeypair],
      }
    );
    print(`user three sign proposal transaction: ${tx4}`);

    groupAccount = await program.account.groupAccount.fetch(group);
    assert.equal(groupAccount.currentManager.toString(), "1");
    assert.equal(groupAccount.currentMember.toString(), "0");
    assert.isFalse(groupAccount.electing);

    memberThreeAccount = await program.account.memberAccount.fetch(memberThree);
    assert.isFalse(memberThreeAccount.inPromotion);
    assert.equal(JSON.stringify(memberThreeAccount.position), JSON.stringify({manager: {}}));

    print("complete submit proposal reelection");
  })


});

function participateRate() {
  this.numerator;
  this.denominator;
}