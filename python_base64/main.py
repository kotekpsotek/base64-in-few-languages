import re
from base64alphabet import BASE64_ALPHABET as B64A, find_char_by_code

"""base64 encoding/decoding interface"""
class Base64Encoding:
    # Encode utf-8 characters to base64
    def encode(dat_t_enc: str):
        # Get 6 biest chunks
        utf_8_bi = ''.join(f"{ord(i):08b}" for i in dat_t_enc) # obtain bites from string
        slice_6b: list = re.findall("\d{0,5}\S", utf_8_bi)
        letters_b64alph = []

        # Get character from base64 alphabet
        for bite6 in slice_6b:
            if len(bite6) < 6:
                bite6 = Base64Encoding._fill_to_number(bite6, 6).join(bite6)

            char_code = int(bite6, 2) # get decimal number from binary substitute
            b64_char = find_char_by_code(char_code)
            
            if b64_char.__len__() == 0:
                raise RuntimeError()

            letters_b64alph.append(b64_char)

        # Add appropriate paddings count
        rest = len(utf_8_bi) % 3
        if rest != 0:
            paddings = []
            for _ in range(0, rest):
                paddings.append("=")
            
            letters_b64alph.append(''.join("="))

        # Return value
        return ''.join(letters_b64alph)
                

    # Get lacked "0" bits to enroll length specified in "to_num" by bites chain placed under "data_evaluated"
    def _fill_to_number(data_evaluated: str, to_num: int) -> str:
        to_fill = []
        for i in range(0, (to_num - len(data_evaluated))):
            to_fill.append("0")

        return ''.join(to_fill)
                

if __name__ == "__main__":
    question = input("Enter data to encode: ")
    encoded = Base64Encoding.encode(question)
    print(encoded)
