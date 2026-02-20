# simple image to text script
#HOW TO USE: enter the file location while executing the script (e.g. "python3 o2t.py image.png")
import sys
import pytesseract

with open("out.txt", "w") as f:
    f.write(pytesseract.image_to_string(sys.argv[1]))

print("Done!")
