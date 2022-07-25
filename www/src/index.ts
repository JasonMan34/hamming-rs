import { corrupt } from "hamming-corruptor";
import { decode } from "hamming-decoder";
import { encode_8_4 } from "hamming-encoder";

interface File {
  content: Uint8Array;
  encoded: Uint8Array;
  corrupted: Uint8Array;
  decodedFixed: Uint8Array;
  decodedUnfixed: Uint8Array;
  text: String;
}

let file: File | undefined;

const fileInput = document.getElementById("file") as HTMLInputElement;
const encodeBtn = document.getElementById("encode") as HTMLButtonElement;
const decodeBtn = document.getElementById("decode") as HTMLButtonElement;
const corruptBtn = document.getElementById("corrupt") as HTMLButtonElement;

fileInput.onchange = async () => {
  let buffer = await fileInput.files?.[0].arrayBuffer();
  if (buffer) {
    let content = new Uint8Array(buffer);
    let encoded = encode_8_4(content);
    let corrupted = corrupt(encoded);
    let decodedFixed = decode(corrupted, true);
    let decodedUnfixed = decode(corrupted, false);
    let text = new TextDecoder().decode(buffer);

    console.log(text);

    file = {
      content,
      encoded,
      corrupted,
      decodedFixed,
      decodedUnfixed,
      text,
    };
  }
};
