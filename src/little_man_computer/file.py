from io import TextIOWrapper
from typing import List, Tuple

from .assemble import mnemonic_compile, numeric_to_instruction
from .constants import FileHeaders
from .exceptions import UnknownFileHeader
from .types import Instruction, MemAddr, MnemonicInstruction


def split_row(raw_row: str, sep=";") -> List[str]:
    """
    split a row into columns

        :param raw_row: the row
        :param sep: what the seperator is, defaults to ";"
        :return: a list of columns
    """
    return raw_row.split(sep)


def sanitize_rows(raw_rows: str) -> List[str]:
    """
    remove blank lines and comments

        :param raw_rows: the rows
        :return: a list of 'cleaned' rows
    """
    cleaned_rows = []

    for row in raw_rows:
        if row != "" and row[0:2] != "//":
            cleaned_rows.append(row.partition("//")[0])

    return cleaned_rows


def load_mnemonic(raw_rows: List[str]) -> Tuple[List[Instruction], Tuple[MemAddr]]:
    """
    load a mnemonic file into Instruction obj

        :param raw_rows: the rows
        :return: the list of instructions and pre-defined memory
    """
    # split the rows into columns
    raw_rows = list(map(split_row, raw_rows))

    mnemonic_instr = [MnemonicInstruction(*row) for row in raw_rows]
    return mnemonic_compile(mnemonic_instr)


def load_numbered(raw_rows: List[str]) -> Tuple[List[Instruction], Tuple[MemAddr]]:
    """
    load a numbered file into Instruction obj

        :param raw_rows:
        :return: the list of instructions and pre-defined memory
    """
    return list(map(numeric_to_instruction, raw_rows)), ()


def load_lmc_file(fp: TextIOWrapper) -> Tuple[List[Instruction], Tuple[MemAddr]]:
    """
    will load a lmc file, allowing the use
    of $mnemonic or $numbered file types

        :param fp: the open file to read from
        :raises UnknownFileHeader: if a unknown header was found
        :return: the list of instructions
    """
    raw_rows = fp.read().replace(" ", "").split("\n")

    # get the header row e.g. $mnemonic -> mnemonic
    header = raw_rows.pop(0).replace("$", "")

    # remove blank rows and comments
    raw_rows = sanitize_rows(raw_rows)

    # decide what type of lmc file it is
    if header == FileHeaders.MNEMONIC:
        return load_mnemonic(raw_rows)

    elif header == FileHeaders.NUMBERED:
        return load_numbered(raw_rows)

    raise UnknownFileHeader(f"Unknown header type {header}")
