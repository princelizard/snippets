# simple image to text script
import sys
import pytesseract

imagePath = sys.argv[1]

with open("out.txt", "w") as f:
    f.write(pytesseract.image_to_string(imagePath))

print("Done!")