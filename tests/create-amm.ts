import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pump } from "../target/types/pump";
import { expect } from "chai";
import { TestValues, createValues, expectRevert } from "./utils";

describe("Create AMM", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Pump as Program<Pump>;

  let values: TestValues;

  beforeEach(() => {
    values = createValues();
  });

  it("Creation", async () => {
    await program.methods
      .createAmm(values.id)
      .accounts({ amm: values.ammKey, admin: values.admin.publicKey })
      .rpc();

    const ammAccount = await program.account.amm.fetch(values.ammKey);
    expect(ammAccount.id.toString()).to.equal(values.id.toString());
    expect(ammAccount.admin.toString()).to.equal(
      values.admin.publicKey.toString()
    );
    expect(ammAccount.fee.toString()).to.equal(values.fee.toString());
  });

  it("Invalid admin", async () => {
    await expectRevert(
      program.methods
        .createAmm(values.id)
        .accounts({ amm: values.ammKey, admin: values.trader.publicKey })
        .rpc()
    );
  });
  it("Invalid fee", async () => {
    await expectRevert(
      program.methods
        .createAmm(values.id)
        .accounts({ amm: values.ammKey, admin: values.admin.publicKey })
        .rpc()
    );
  });
});
