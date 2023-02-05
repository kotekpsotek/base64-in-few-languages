import re
from base64alphabet import BASE64_ALPHABET as B64A, find_char_by_code

"""base64 encoding/decoding interface"""
class Base64Encoding:
    # Encode utf-8 characters to base64
    def encode(dat_t_enc: str):
        # Get 6 biest chunks
        utf_8_bi = ''.join(f"{ord(i):08b}" for i in dat_t_enc) # obtain bites from string
        slice_6b: list[str] = re.findall("\d{0,5}\S", utf_8_bi)
        letters_b64alph = []

        # Get character from base64 alphabet
        for bite6 in slice_6b:
            if len(bite6) < 6:
                bite6 = f"{bite6}{Base64Encoding._fill_to_number(bite6, 6)}" # lacking 0 must be attached to start (because base64 to create must obtain 4 pieces of 6bit binary numbers from 3 8 bit binary numbers and decoding process must be compatible in reverse to it)

            char_code = int(bite6, 2) # get decimal number from binary substitute
            b64_char = find_char_by_code(char_code)
            
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
    

    # Decode from base64 to utf-8 (reverse process)
    def decode(to_decode: str) -> str:
        # Remove base64 paddiings
        to_decode = to_decode.replace("=", "")

        result: str = ""

        # Get 6 character binary number from base64 character code (obtained from BASE64 alphabet)
        char_bin_6b = ""

        for char in to_decode:
            char_code: int | None = None
            for [code, char_ev] in B64A:
                if char == char_ev:
                    char_code = code
                    break

            if char_code != None:
                char_code_binary = f'{code:06b}'
                char_bin_6b += char_code_binary

        # Do 8 character binary number from creted in most above part binary chain consisting of the 6 character binary numbers
        char_bin_8b_slice: list[str] = re.findall("\d{0,7}\S", char_bin_6b)

        # Will be make utf-8 character created from utf-8 code obtained from 8 character binary number from above step
        for slice_8b in char_bin_8b_slice:
            if len(slice_8b) < 8 and slice_8b.__contains__("1"): # expand not equal to decimal "0" binary slice to 8 bits by add lacking zero ones to the start of it (because in this manner we don't increase number as in case when we add '0' to the end of string)
                slice_8b = f"{Base64Encoding._fill_to_number(slice_8b, 6)}{slice_8b}"
            elif len(slice_8b) < 8 and not slice_8b.__contains__("1"): # ignore slice shorther then 8 numbers which is equal to decimal "0"
                continue

            num_decimal = int(slice_8b, 2) # create decimal number from binary which is code from binary number
            bytes = bytearray([num_decimal]) # code must be casted as "bytearray" to obtain later character using python interface hidden under the hood 
            char = bytes.decode("utf-8") # create utf-8 character from it's code using hidden under the hood interface assigned to "bytearray" type
            
            # Add extracted character to decoding result
            result += char

        # Return decoding base64 result
        return result
            
    # Get lacked "0" bits to enroll length specified in "to_num" by bites chain placed under "data_evaluated"
    def _fill_to_number(data_evaluated: str, to_num: int) -> str:
        to_fill = []
        for _ in range(0, to_num - len(data_evaluated)):
            to_fill.append("0")

        return ''.join(to_fill)

if __name__ == "__main__":
    question = input("Enter data to encode: ")
    encoded = Base64Encoding.encode(question)
    print("Encoded: ", encoded)
    decoded = Base64Encoding.decode(encoded)
    print("Decoded: ", decoded)
