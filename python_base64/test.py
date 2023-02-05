""" Base64 decoder test suite """

import unittest
from main import Base64Encoding

# Use command "python -m unittest ./test.py" to "run all test"
# Use command "python ./test.py Tests.method_name" to run only one specified test or more then one when you pass multiple method names base on schema CLassName.method_class name like here: "python ./test.py Tests.method_name Tests.method_name2"
class Tests(unittest.TestCase):
    # + Each test class method name must begin with "Test"
    # + Method names must be different to run each

    # Testing on values
    TEST_ENCOING = "Today is really good weahter"
    TEST_DECODING = "VG9kYXkgaXMgcmVhbGx5IGdvb2Qgd2VhaHRlcg="

    # Test expectations
    EXPECTED_ENCODING = "VG9kYXkgaXMgcmVhbGx5IGdvb2Qgd2VhaHRlcg="
    EXPECTED_DECODING = "Today is really good weahter"

    # base64 encoding test
    def test_encoding(self):
        encoding_result = Base64Encoding.encode(self.TEST_ENCOING)
        self.assertEqual(encoding_result, self.EXPECTED_ENCODING, f"Result of encoding to base64 is different then expected value.\nEncoding result: {self.TEST_ENCOING},\nExpected result: {self.EXPECTED_ENCODING}")

    # base64 decoding test
    def test_decoding(self):
        encoding_result = Base64Encoding.decode(self.TEST_DECODING)
        self.assertEqual(encoding_result, self.EXPECTED_DECODING, f"Result of encoding to base64 is different then expected value.\nEncoding result: {self.TEST_DECODING},\nExpected result: {self.EXPECTED_DECODING}")

# You can run single/selected test/tests by you thanks to below snippest
if __name__ == "__main__":
    unittest.main()