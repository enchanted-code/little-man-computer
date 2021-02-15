from typing import List, Optional

from .types import MemAddr, Instruction


class LittleManComputer:
    __program_counter = 0
    __accumulator = 0
    # 0,99 of memory (100 bits)
    __memory = [0 for i in range(100)]
    # use seperate memory for storing instructions
    __instructions = None

    def __init__(self, instructions: List[Instruction], dats: List[MemAddr]):
        self.__instructions = instructions
        for dat in dats:
            self.__memory[dat.addr] = dat.value

    def incr_program_counter(self, next_inst: Optional[int] = None):
        if next_inst is None:
            self.__program_counter += 1
        else:
            self.__program_counter = next_inst

    @property
    def curr_instr(self):
        return self.__instructions[self.__program_counter]

    @property
    def accumulator(self):
        return self.__accumulator

    @accumulator.setter
    def accumulator(self, new_val: int):
        self.__accumulator = new_val
        if self.__accumulator > 999 or self.__accumulator < -1:
            # show negative flag for overflow
            self.__accumulator = -1

    def load(self, addr: int):
        """
        load value from addr in
        memory into the accumulator

            :param addr: the memory addr
        """
        self.accumulator = self.__memory[addr]

    def store(self, addr: int):
        """
        store value into memory from the accumulator

            :param addr: the memory addr
        """
        self.__memory[addr] = self.accumulator

    def add(self, addr: int):
        """
        perform an add operation on
        accumulator and a value in memory

            :param addr: the memory addr
        """
        self.accumulator += self.__memory[addr]

    def sub(self, addr: int):
        """
        perform an minus operation on
        accumulator and a value in memory

            :param addr: the memory addr
        """
        self.accumulator -= self.__memory[addr]
