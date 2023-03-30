from argparse import BooleanOptionalAction
from datetime import datetime

from solana.rpc.api import Client
from solana.publickey import PublicKey
from spl.token.instructions import get_associated_token_address
from base64 import b64decode, b64encode
from base58 import b58decode, b58encode
from construct import RepeatUntil, Struct, BytesInteger, Bytes, Adapter, this, PascalString, Array, Union
from construct import setGlobalPrintFullStrings


setGlobalPrintFullStrings(True)
# client = Client("https://api.mainnet-beta.solana.com")
# client = Client("https://api.devnet.solana.com")
client = Client("https://api.testnet.solana.com")
# client = Client("http://127.0.0.1:8899")
program_id = PublicKey("9fAvfoKEWUUSRmDvfeVoZ1HXKGDnkUB8rguYUMeUcs6W")

class PubkeyAdapter(Adapter):
    def _decode(self, obj, context, path):
        return PublicKey(obj)
    def _encode(self, obj, context, path):
        return bytes(obj)

class TimestampAdapter(Adapter):
    def _decode(self, obj, context, path):
        return datetime.fromtimestamp(obj)
    def _encode(self, obj, context, path):
        return obj.timestamp()

class BooleanAdapter(Adapter):
    def _decode(self, obj, ctx, path):
        if obj == 0:
            return False
        elif obj == 1:
            return True

class PositionAdapter(Adapter):
    def _decode(self, obj, ctx, path):
        if obj == 0:
            return "Member"
        elif obj == 1:
            return "Manager"

group_account_schema = Struct(
    Bytes(8),
    "seed" / BytesInteger(1),
    "electing" / BooleanAdapter(BytesInteger(1)),
    "freeze" / BooleanAdapter(BytesInteger(1)),
    "rate" / Struct(
        "numerator" / BytesInteger(1),
        "denominator" / BytesInteger(1),
    ),
    "update" / BooleanAdapter(BytesInteger(1)),
    Bytes(2),
    "maxManager" / BytesInteger(4, swapped=True),
    "currentManager" / BytesInteger(4, swapped=True),
    "currentMember" / BytesInteger(4, swapped=True),
    "proposals" / BytesInteger(4, swapped=True),
    "index" / BytesInteger(4, swapped=True),
    "sponsor" / PubkeyAdapter(Bytes(32)),
    "admin" / PubkeyAdapter(Bytes(32)),
)

class ProposalTypeAdapter(Adapter):
    def _decode(self, obj, ctx, path):
        kind = BytesInteger(1).parse(obj)
        if kind == 0:
            return "Upgrade"
        elif kind == 1:
            return "Downgrade"
        elif kind == 2:
            return Struct(
                Bytes(4),
                "maxManager" / BytesInteger(4, swapped=True),
            ).parse(obj)
        elif kind == 3:
            return "ReElection"
        elif kind == 4:
            return Struct(
                Bytes(8),
                "mint" / PubkeyAdapter(Bytes(32)),
                "receiver" / PubkeyAdapter(Bytes(32)),
                "amount" / BytesInteger(8, swapped=True),
            ).parse(obj)

class ProposalStatusAdapter(Adapter):
    def _decode(self, obj, ctx, path):
        status = BytesInteger(1).parse(obj)
        if status == 0:
            return "Progressing"
        elif status == 1:
            return "Passed"
        elif status == 2:
            return "Rejected"
        elif status == 3:
            return "Updated"

proposal_account_schema = Struct(
    Bytes(8),
    "submitter" / PubkeyAdapter(Bytes(32)),
    "beneficiary" / PubkeyAdapter(Bytes(32)),
    "bene_member" / PubkeyAdapter(Bytes(32)),
    "group" / PubkeyAdapter(Bytes(32)),
    "positive" / BytesInteger(4, swapped=True),
    "negative" / BytesInteger(4, swapped=True),
    "limit" / BytesInteger(8, swapped=True),
    "deadline" / TimestampAdapter(BytesInteger(8, signed=True, swapped=True)),
    "revoke_timeout" / TimestampAdapter(BytesInteger(8, signed=True, swapped=True)),
    "close_timeout" / TimestampAdapter(BytesInteger(8, signed=True, swapped=True)),
    "type" / ProposalTypeAdapter(Bytes(80)),
    "status" / ProposalStatusAdapter(Bytes(16)),
)

member_account_schema = Struct(
    Bytes(8),
    "position" / PositionAdapter(BytesInteger(1)),
    "in_promotion" / BooleanAdapter(BytesInteger(1)),
    "in_withdraw" / BooleanAdapter(BytesInteger(1)),
    "group" / PubkeyAdapter(Bytes(32)),
    "funder" / PubkeyAdapter(Bytes(32)),
    "owner" / PubkeyAdapter(Bytes(32)),
)

admin_account_schema = Struct(
    Bytes(8),
    "seed" / BytesInteger(1),
    "current" / BytesInteger(1),
    "initialized" / BooleanAdapter(BytesInteger(1)),
    Bytes(1),
    "group" / BytesInteger(4, swapped=True),
    "token_mint" / PubkeyAdapter(Bytes(32)),
    "admins" / Array(10, PubkeyAdapter(Bytes(32))),
)

class ProposalEventTypeAdapter(Adapter):
    def _decode(self, obj, ctx, path):
        if obj == 0:
            return "Upgrade"
        elif obj == 1:
            return "Downgrade"
        elif obj == 2:
            return "Withdraw"
        elif obj == 3:
            return "UpdateGroup"
        elif obj == 4:
            return "ReElection"

submit_proposal_event_schema = Struct(
    Bytes(8),
    "ptype" / ProposalEventTypeAdapter(BytesInteger(1)),
    "submitter" / PubkeyAdapter(Bytes(32)),
    "submitter_member" / PubkeyAdapter(Bytes(32)),
    "beneficiary" / PubkeyAdapter(Bytes(32)),
    "bene_member" / PubkeyAdapter(Bytes(32)),
    "group" / PubkeyAdapter(Bytes(32)),
    "proposal" / PubkeyAdapter(Bytes(32)),
    "deadline" / TimestampAdapter(BytesInteger(8, signed=True, swapped=True)),
    "label" / PascalString(BytesInteger(4, swapped=True), "utf8"),
)

def parse_group_account(group: PublicKey):
    data = parse_account(group)
    return group_account_schema.parse(b64decode(data))

def parse_proposal_account(proposal: PublicKey):
    data = parse_account(proposal)
    return proposal_account_schema.parse(b64decode(data))

def parse_member_account(member: PublicKey):
    data = parse_account(member)
    return member_account_schema.parse(b64decode(data))

def parse_admin_account(admin: PublicKey):
    data = parse_account(admin)
    return admin_account_schema.parse(b64decode(data))

def parse_account(pubkey: PublicKey):
    resp = client.get_account_info(pubkey, encoding="jsonParsed")
    if resp['result']['value'] == None:
        raise ValueError(f"Account {pubkey} doesn't exist")
    return resp['result']['value']['data'][0]

def main():
    # group = PublicKey("H1Uy3xvNo4JsS7SGkBCPigwMoFwqbs6vvWrALfHeAAgB")
    # group = PublicKey("2sDTuke5ZzL6K1QgcgibgxpCCLHfTBEdmKZpDay7fTGY")
    # print(parse_group_account(group))

    proposal = PublicKey("33eXB8SGZtuxmZ2sE3KfLo7cjd2k1F5az1BxkrrdZ35i")
    # print(parse_proposal_account(proposal))

    member = PublicKey("6hEC5H6yCtbazrBMvDHaHknc1h9KMi8UidHwfJECcPZw")
    # print(parse_member_account(member))

    # admin = PublicKey("2fRjtTakd4hU4PoTT21DFAJ488iHNHbBqYq2SZ1VgFeG")
    # print(parse_admin_account(admin))


if __name__ == "__main__":
    main()
