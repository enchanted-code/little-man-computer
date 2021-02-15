from typing import List, Tuple

from .constants import MNEMONICS
from .exceptions import LoopNotDefined, VariableNotDefined
from .types import Instruction, MemAddr, MnemonicInstruction


def numeric_to_instruction(combined: str) -> Instruction:
    """
    convert a numeric instruction
    into a Instruction obj

        :param combined: the opcode and addr
    """
    return Instruction(int(combined[0]), int(combined[1:]))


def load_dats(program: List[MnemonicInstruction]) -> dict:
    """
    finds any dat values and assigns a memory address

        :param program: the program
        :return: any dats that were found
    """
    dats = {}
    # -1 indicates no memory was used
    max_memory_addr = -1
    # find variables
    for i in range(len(program)-1, 0, -1):
        if program[i].mnemonic == "HLT":
            # we have reached the end of variables
            break
        max_memory_addr += 1
        if program[i].variable.isnumeric():
            # if value is set
            dats[program[i].label] = MemAddr(int(program[i].variable), max_memory_addr)
        else:
            # if value was not set used default 0
            dats[program[i].label] = MemAddr(0, max_memory_addr)
    return dats


def load_loops(program: List[MnemonicInstruction]) -> dict:
    """
    find any loop labels and store their addresses

        :param program: the program
        :return: any loops that were found
    """
    loops = {}
    for i in range(len(program)):
        # check whether instruction is a loop label
        if program[i].label and program[i].mnemonic != "DAT":
            loops[program[i].label] = i
    return loops


def mnemonic_compile(
    program: List[MnemonicInstruction]) -> Tuple[List[Instruction], Tuple[MemAddr]]:
    """
    compile mnemonic instructions into instructions
    using opcodes and addresses

        :param program: the program to compile,
                        given as a list of mnemonic instructions
        :raises VariableNotDefined: when a variable in a
                                    instruction has not been defined
        :raises LoopNotDefined: when a loop label in a
                                instruction has not been defined
        :raises ValueError: when there is a unknown mnemonic value
        :return: the instructions and pre-set memory addresses
    """
    # get all dat values and assign memory addr
    dats = load_dats(program)
    # get all loop labels and find their instruction code
    loop_labels = load_loops(program)

    instructions = []
    for instr in program:
        if instr.mnemonic == "DAT":
            continue
        elif instr.mnemonic == "HLT":
            instr = Instruction(MNEMONICS[instr.mnemonic])
            instructions.append(instr)
        elif instr.mnemonic == "INP":
            instr = Instruction(MNEMONICS[instr.mnemonic], 1)
            instructions.append(instr)
        elif instr.mnemonic == "OUT":
            instr = Instruction(MNEMONICS[instr.mnemonic], 2)
            instructions.append(instr)
        elif instr.mnemonic in ("ADD", "SUB", "STA", "LDA"):
            try:
                instr = Instruction(
                    MNEMONICS[instr.mnemonic],
                    dats[instr.variable].addr)
                instructions.append(instr)
            except KeyError as err:
                raise VariableNotDefined(f"Unknown variable name given: {instr.variable}") from err
        elif instr.mnemonic in ("BRA", "BRZ", "BRP"):
            try:
                instr = Instruction(
                    MNEMONICS[instr.mnemonic],
                    loop_labels[instr.variable])
                instructions.append(instr)
            except KeyError as err:
                raise LoopNotDefined(f"Unknown loop name given: {instr.variable}") from err
        else:
            raise ValueError(f"Unknown mnemonic: {instr.mnemonic}")
    return instructions, tuple(dats.values())
