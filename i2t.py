# simple image to text script
import sys
import pytesseract

with open("out.txt", "w") as f:
    f.write(pytesseract.image_to_string(sys.argv[1]))

print("Done!")
