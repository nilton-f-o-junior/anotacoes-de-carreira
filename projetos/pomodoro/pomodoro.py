import time
import os
import sys
import threading

#
try:
    from colorama import init, Fore, Style
    init(autoreset=True)
    HAS_COLOR = True
except ImportError:
    class Fore:
        RED = GREEN = YELLOW = CYAN = MAGENTA = WHITE = ""
    class Style:
        BRIGHT = RESET_ALL = ""
    HAS_COLOR = False

#
DIGITS = {
    '0': ["  ###  ", " #   # ", " #   # ", " #   # ", "  ###  "],
    '1': ["   #   ", "  ##   ", "   #   ", "   #   ", " ##### "],
    '2': [" ##### ", "     # ", " ##### ", " #     ", " ##### "],
    '3': [" ##### ", "     # ", " ##### ", "     # ", " ##### "],
    '4': [" #   # ", " #   # ", " ##### ", "     # ", "     # "],
    '5': [" ##### ", " #     ", " ##### ", "     # ", " ##### "],
    '6': [" ##### ", " #     ", " ##### ", " #   # ", " ##### "],
    '7': [" ##### ", "     # ", "    #  ", "   #   ", "   #   "],
    '8': [" ##### ", " #   # ", " ##### ", " #   # ", " ##### "],
    '9': [" ##### ", " #   # ", " ##### ", "     # ", " ##### "],
    ':': ["       ", "   #   ", "       ", "   #   ", "       "],
}

#
WORK_COLOR  = Fore.GREEN  + Style.BRIGHT
BREAK_COLOR = Fore.CYAN   + Style.BRIGHT
PAUSE_COLOR = Fore.YELLOW + Style.BRIGHT
WARN_COLOR  = Fore.RED    + Style.BRIGHT
RESET       = Style.RESET_ALL

# titulo
def make_mode_line(icon: str, label: str, badge_color: str, cycle_num: int, dim: str = "") -> str:
    DIM = dim if dim else "\033[2m"
    badge = f"{badge_color}[ {label} ]{RESET}"
    sep   = f"{DIM}│{RESET}"
    ciclo = f"{DIM}Ciclo #{cycle_num}{RESET}"
    return f"  {icon}  {badge}  {sep}  {ciclo}"

# barra de progresso
def make_bar(elapsed: int, total: int, width: int = 42, color: str = "") -> str:
    filled = min(int(elapsed / total * width), width)
    bar    = "▰" * filled + "▱" * (width - filled)
    pct    = int(elapsed / total * 100)
    return f"{color}{bar}  {pct:3d}%{RESET}"

# relogio
def format_ascii_time(mins: int, secs: int, color: str = "") -> str:
    time_str = f"{mins:02d}:{secs:02d}"
    rows = ["", "", "", "", ""]
    for ch in time_str:
        for i, line in enumerate(DIGITS[ch]):
            rows[i] += line + " "
    return "\n".join(f"  {color}{row}{RESET}" for row in rows)

#
def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def beep():
    print("\a", end="", flush=True)

#
def _make_key_listener(pause_event: threading.Event, skip_event: threading.Event):
    if os.name == 'nt':
        import msvcrt
        def _listen():
            while True:
                if msvcrt.kbhit():
                    key = msvcrt.getwch().lower()
                    if key == 'p':
                        if pause_event.is_set(): pause_event.clear()
                        else: pause_event.set()
                    elif key in ('\r', '\n', ' '):
                        skip_event.set()
                time.sleep(0.05)
    else:
        import tty, termios, select
        def _listen():
            fd  = sys.stdin.fileno()
            old = termios.tcgetattr(fd)
            try:
                tty.setcbreak(fd)
                while True:
                    if select.select([sys.stdin], [], [], 0.05)[0]:
                        key = sys.stdin.read(1).lower()
                        if key == 'p':
                            if pause_event.is_set(): pause_event.clear()
                            else: pause_event.set()
                        elif key in ('\r', '\n', ' '):
                            skip_event.set()
            finally:
                termios.tcsetattr(fd, termios.TCSADRAIN, old)

    t = threading.Thread(target=_listen, daemon=True)
    return t

# ciclo
def run_cycle(minutes: int, label: str, color: str,
              pause_event: threading.Event, cycle_num: int):
    total_secs = minutes * 60
    remaining  = total_secs

    while remaining >= 0:
        elapsed = total_secs - remaining

        if pause_event.is_set():
            clear_screen()
            mins, secs = divmod(remaining, 60)
            print("\n     " + make_mode_line("⏸", "PAUSADO", PAUSE_COLOR, cycle_num))
            print("\n" + format_ascii_time(mins, secs, PAUSE_COLOR))
            print(f"\n  {make_bar(elapsed, total_secs, color=PAUSE_COLOR)}")
            print(f"\n       {PAUSE_COLOR}[P] Retomar   |   [Ctrl+C] Sair{RESET}")
            while pause_event.is_set():
                time.sleep(0.1)
            continue

        clear_screen()
        mins, secs     = divmod(remaining, 60)
        display_color  = WARN_COLOR if remaining <= 60 else color

        print("\n       " + make_mode_line("▶", label, display_color, cycle_num))
        print("\n" + format_ascii_time(mins, secs, display_color))
        print(f"\n  {make_bar(elapsed, total_secs, color=display_color)}")
        print(f"\n        [P] Pausar   |   [Ctrl+C] Sair")

        time.sleep(1)
        remaining -= 1

# time
def pomodoro_timer(work_mins: int = 25, break_mins: int = 5):
    pause_event = threading.Event()
    skip_event = threading.Event()
    _make_key_listener(pause_event, skip_event).start()
    cycle = 1

    try:
        while True:
            run_cycle(work_mins, "FOCO", WORK_COLOR, pause_event, cycle)
            beep()
            
            skip_event.clear()
            while not skip_event.is_set():
                clear_screen()
                print(f"\n  {WORK_COLOR}✔  Foco #{cycle} concluído!{RESET}")
                print(f"\n  Pressione [ESPAÇO] ou [ENTER] para iniciar a pausa...")
                time.sleep(0.2)
            
            run_cycle(break_mins, "PAUSA", BREAK_COLOR, pause_event, cycle)
            beep()
            
            skip_event.clear()
            while not skip_event.is_set():
                clear_screen()
                print(f"\n  {BREAK_COLOR}✔  Pausa concluída!{RESET}")
                print(f"\n  Pressione [ESPAÇO] ou [ENTER] para voltar ao trabalho...")
                time.sleep(0.2)

            cycle += 1

    except KeyboardInterrupt:
        clear_screen()
        print(f"\n  {WARN_COLOR}Pomodoro encerrado. Até a próxima! 🍅{RESET}\n")

# entrada
if __name__ == "__main__":
    work   = int(sys.argv[1]) if len(sys.argv) > 1 else 25
    break_ = int(sys.argv[2]) if len(sys.argv) > 2 else 5
    pomodoro_timer(work, break_)
