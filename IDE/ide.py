from tkinter import filedialog, TclError
import customtkinter as ctk
from pathlib import Path
import sys

path = str(Path(__file__).resolve().parent.parent)
sys.path.append(path)

root = ctk.CTk()
root.title("OVER IDE")
root.geometry("1920x1080")
root.grid_columnconfigure(0, weight=1)
root.grid_rowconfigure(0, weight=1)

from over.lexer import lex
from over.parser import parse
from over.evaluator import evaluate
from over.exceptions import InvalidExpressionError

class IDE:
    def __init__(self):
        self.filename = None
        self.memory = {}
        self.current_screen = "menu"

ide = IDE()

def run(*_):
    ide.memory = {}
    try:
        console_text.delete("1.0", "end")
        txt = text.get("1.0", "end-1c")
        tokens = lex(txt)
        node = parse(tokens, txt)
        result = evaluate(node, ide.memory)
        if result is not None:
            console_text.insert("1.0", str(result))
        console_frame.grid(column=0, row=2, sticky="nsew", padx=10, pady=10)
    except InvalidExpressionError as msg:
        console_text.insert("1.0", str(msg))
        console_frame.grid(column=0, row=2, sticky="nsew", padx=10, pady=10)

def text_delete(*_):
    text.delete("1.0", "end")

def insert(txt: str):
    text.insert("1.0", txt)

def save_file(*_):
    if ide.filename is not None:
        txt = text.get("1.0", "end-1c")
        with open(ide.filename, "w", encoding="utf-8") as file:
            file.write(txt)
            return
    filename = filedialog.asksaveasfilename(filetypes=[("Over files", "*.over"), ("all files", "*.*")], title="save Over file", defaultextension=".over")
    if filename:
        ide.filename = filename
        txt = text.get("1.0", "end-1c")
        with open(filename, "w", encoding="utf-8") as file:
            file.write(txt)

def open_file_menu(*_):
    filename = filedialog.askopenfilename(filetypes=[("Over files", "*.over"), ("all files", "*.*")], title="open Over file", defaultextension=".over")
    if filename:
        ide.filename = filename
        with open(filename, "r", encoding="utf-8") as file:
            txt = file.read()
        show_editor()
        insert(txt)

def open_file(*_):
    filename = filedialog.askopenfilename(filetypes=[("Over files", "*.over"), ("all files", "*.*")], title="open Over file", defaultextension=".over")
    if filename:
        ide.filename = filename
        with open(filename, "r", encoding="utf-8") as file:
            txt = file.read()
        text_delete()
        insert(txt)

def new_file(*_):
    text_delete()
    ide.filename = None

def show_editor(*_):
    ide.current_screen = "editor"
    menu_main_frame.grid_forget()
    editor_main_frame.grid(row=0, column=0, sticky="nsew")

def show_menu(*_):
    ide.current_screen = "menu"
    editor_main_frame.grid_forget()
    menu_main_frame.grid(row=0, column=0, sticky="nsew")

def close_console(*_):
    console_frame.grid_forget()

def select_all(*_):
    text.tag_add("sel", "1.0", "end-1c")

def copy_text(*_):
    try:
        selected_text = text.get("sel.first", "sel.last")
    except TclError:
        return
    root.clipboard_clear()
    root.clipboard_append(selected_text)
    root.update()

def paste_text(*_):
    try:
        clipboard_text = root.clipboard_get()
    except TclError:
        return
    text.insert("insert", clipboard_text)

def cut_text(*_):
    try:
        selected_text = text.get("sel.first", "sel.last")
    except TclError:
        return
    root.clipboard_clear()
    root.clipboard_append(selected_text)
    root.update()
    text.delete("sel.first", "sel.last")

def undo(*_):
    text.event_generate("<<Undo>>")

keycodes = {
    65: select_all,
    67: copy_text,
    68: text_delete,
    79: open_file,
    82: run,
    83: save_file,
    86: paste_text,
    88: cut_text,
    90: undo
}

def shortcuts(event):
    if not (event.state & 0x0004):
        return
    keycode = event.keycode
    if ide.current_screen == "menu":
        if keycode == 79:
            open_file_menu()
    elif ide.current_screen == "editor":
        if keycode in keycodes:
            keycodes[keycode]()
        else:
            return
    return "break"

root.bind_all("<KeyPress>", shortcuts)

menu_main_frame = ctk.CTkFrame(root)
menu_main_frame.grid_rowconfigure(0, weight=1)
menu_main_frame.grid_rowconfigure(1, weight=1)
menu_main_frame.grid_columnconfigure(0, weight=1)

header_menu_frame = ctk.CTkFrame(menu_main_frame)
button_menu_frame = ctk.CTkFrame(menu_main_frame)

header_menu_frame.grid(row=0, column=0, padx=10, pady=10)
button_menu_frame.grid(row=1, column=0, padx=10, pady=10)

label = ctk.CTkLabel(header_menu_frame, text="MAIN MENU", font=("Arial", 24))
label.grid(row=0, column=0, sticky="nsew", padx=10, pady=10)

start_button = ctk.CTkButton(button_menu_frame, text="start", command=show_editor, fg_color="#000000", hover_color="#1A1A1A")
start_button.grid(row=1, column=0, sticky="nsew", padx=10, pady=10)

open_button = ctk.CTkButton(button_menu_frame, text="open", command=open_file_menu, fg_color="#000000", hover_color="#1A1A1A")
open_button.grid(row=1, column=1, sticky="nsew", padx=10, pady=10)


editor_main_frame = ctk.CTkFrame(root)

button_editor_frame = ctk.CTkFrame(editor_main_frame)
editor_frame = ctk.CTkFrame(editor_main_frame)
console_frame = ctk.CTkFrame(editor_main_frame)

editor_main_frame.grid_columnconfigure(0, weight=1)
editor_main_frame.grid_rowconfigure(0, weight=0)
editor_main_frame.grid_rowconfigure(1, weight=1)
editor_main_frame.grid_rowconfigure(2, weight=0)

button_editor_frame.grid_columnconfigure(0, weight=1)
button_editor_frame.grid_columnconfigure(1, weight=1)
button_editor_frame.grid_columnconfigure(2, weight=1)
button_editor_frame.grid_columnconfigure(3, weight=1)

editor_frame.grid_columnconfigure(0, weight=1)
editor_frame.grid_rowconfigure(0, weight=1)

console_frame.grid_rowconfigure(0, weight=1)
console_frame.columnconfigure(0, weight=1)

button_editor_frame.grid(row=0, column=0,sticky="nsew", padx=10, pady=10)
editor_frame.grid(row=1, column=0,sticky="nsew", padx=10, pady=10)

output_button = ctk.CTkButton(button_editor_frame, text="run", command=run, fg_color="#000000", hover_color="#1A1A1A")
output_button.grid(row=0, column=0, sticky="nsew", padx=10, pady=10)

new_button = ctk.CTkButton(button_editor_frame, text="new", command=new_file, fg_color="#000000", hover_color="#1A1A1A")
new_button.grid(row=0, column=1, sticky="nsew", padx=10, pady=10)

save_button = ctk.CTkButton(button_editor_frame, text="save", command=save_file, fg_color="#000000", hover_color="#1A1A1A")
save_button.grid(row=0, column=2, sticky="nsew", padx=10, pady=10)

open_button = ctk.CTkButton(button_editor_frame, text="open", command=open_file, fg_color="#000000", hover_color="#1A1A1A")
open_button.grid(row=0, column=3, sticky="nsew", padx=10, pady=10)

text = ctk.CTkTextbox(editor_frame)
text.grid(row=0, column=0, sticky="nsew", padx=10, pady=10)
text.configure(font=("Arial", 15))

console_header = ctk.CTkFrame(console_frame, width=5, cursor="sb_h_double_arrow")
console_header.grid(row=0, column=0, sticky="nsew", padx=10, pady=10)

console_text = ctk.CTkTextbox(console_frame, height=200)
console_text.grid(row=0, column=0, sticky="nsew", padx=10, pady=10)
console_text.configure(font=("Arial", 15))

close_button = ctk.CTkButton(console_frame, text="X", command=close_console, fg_color="#000000", hover_color="#1A1A1A")
close_button.grid(row=0, column=1, sticky="n", padx=10, pady=10)

show_menu()
root.mainloop()
