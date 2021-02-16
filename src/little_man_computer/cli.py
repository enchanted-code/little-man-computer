from typing import List

from .core import LittleManComputer
from .types import MemAddr, Instruction


def run(instructions: List[Instruction], dats=List[MemAddr]):
    LMC = LittleManComputer(instructions, dats)

    while True:
        # use for shorthand
        opcode = int(LMC.curr_instr.opcode)
        addr = int(LMC.curr_instr.addr)

        if opcode == 1:
            # ADD
            LMC.add(addr)

        elif opcode == 2:
            # SUB
            LMC.sub(addr)

        elif opcode == 3:
            # STA
            LMC.store(addr)

        elif opcode == 5:
            # LDA
            LMC.load(addr)

        elif opcode == 6:
            # BRA
            LMC.incr_program_counter(addr)
            continue

        elif opcode == 7:
            # BRZ
            if LMC.accumulator == 0:
                LMC.incr_program_counter(addr)
                continue

        elif opcode == 8:
            # BRP
            if LMC.accumulator >= 0:
                LMC.incr_program_counter(addr)
                continue

        elif opcode == 9:
            if addr == 1:
                # INP
                user_in = input("<<< ")
                if len(user_in) > 3:
                    raise ValueError("input can only be 3 digits 0-999")
                LMC.accumulator = int(user_in)
            elif addr == 2:
                # OUT
                print(f">>> {LMC.accumulator}")

        elif opcode == 0:
            # HLT
            break

        else:
            raise ValueError(f"Unknown Instruction: {opcode} {addr}")

        # increment counter by 1
        LMC.incr_program_counter()
