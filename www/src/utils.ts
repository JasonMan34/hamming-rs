export const downloadFile = (name: string, content: Uint8Array) => {
  const element = document.createElement("a");
  document.body.appendChild(element);
  element.setAttribute("style", "display: none");

  const blob = new Blob([content], { type: "octet/stream" });
  const url = window.URL.createObjectURL(blob);
  element.href = url;
  element.download = name;
  element.click();
  window.URL.revokeObjectURL(url);

  document.body.removeChild(element);
};

export class FileName {
  name: string;
  extension: string;

  constructor(name: string) {
    const extensionIndex = name.lastIndexOf(".");
    this.name = name.substring(0, extensionIndex);
    this.extension = name.substring(extensionIndex + 1);
  }

  appendName(str: string) {
    this.name += str;
    return this;
  }

  setExtension(ext: string) {
    this.extension = ext;
    return this;
  }

  getFileName() {
    return this.name + "." + this.extension;
  }
}

export const isText = (str: string) => {
  return /^[\s!-~א-ת]+$/.test(str.substring(0, 100));
};
