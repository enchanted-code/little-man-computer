from dataclasses import dataclass
from typing import Optional


@dataclass
class Instruction:
    opcode: int
    addr: Optional[int] = 0


@dataclass
class MemAddr:
    value: int
    addr: int


@dataclass
class MnemonicInstruction:
    label: str
    mnemonic: str
    variable: str
