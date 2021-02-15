from io import TextIOWrapper
from typing import List, Tuple

from .assemble import mnemonic_compile
from .types import Instruction, MemAddr, MnemonicInstruction


def load_mnemonic(fp: TextIOWrapper) -> Tuple[List[Instruction], Tuple[MemAddr]]:
    """
    load a mnemonic file into Instruction obj

        :param fp: the open file to read from
        :return: the list of instructions
    """
    raw = fp.read().replace(" ", "")
    raw_rows = [row.split(";") for row in raw.split("\n")]

    if raw_rows[-1] == ['']:
        del raw_rows[-1]
    mnemonic_instr = [MnemonicInstruction(*row) for row in raw_rows]
    return mnemonic_compile(mnemonic_instr)
