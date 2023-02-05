import { describe, expect, test } from "vitest";
import Base64 from "./index"


describe("Base64 condig decoding from utf-8", () => {
    const { encode, decode } = Base64;

    test("base64 encoding", () => {
        const dataToEncode = "Data to encode";
        const encoded = encode(dataToEncode);
        expect(encoded).toBeTypeOf("string");

        console.log("Encoded data: ", encoded);
    });

    test("base64 decoding", () => {
        const dataToDecode = "eW91J3JlIGNvb2w="
        const decoded = decode(dataToDecode);
        expect(decoded).toBeTypeOf("string");

        console.log("Decoded data: ", decoded);
    });
})
