import { watermark_image, WatermarkedImageWeb } from 'react-image';
import { WatermarkedImage } from './image';

const mimeRegex = /^data:image\/[a-zA-Z0-9]+;base64,/;

const removeMime = (dataUri: string) => dataUri.replace(mimeRegex, '');

export const toDataURL = (url: string) =>
  new Promise<string>((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.onload = function () {
      const reader = new FileReader();
      reader.onloadend = function () {
        resolve(reader.result as string);
      };
      reader.readAsDataURL(xhr.response);
    };
    xhr.open('GET', url);
    xhr.responseType = 'blob';
    xhr.send();
  });

const decode = (base64_string: string) =>
  Uint8Array.from(atob(base64_string), c => c.charCodeAt(0));

export const base64ToUInt8Array = (dataUri: string): Uint8Array =>
  decode(removeMime(dataUri));

export const toWatermarkedImage = (
  watermarkedImageWeb: WatermarkedImageWeb,
): WatermarkedImage => {
  return {
    width: watermarkedImageWeb.get_width(),
    height: watermarkedImageWeb.get_height(),
    format: watermarkedImageWeb.get_format(),
    original: watermarkedImageWeb.get_original().slice(),
    watermarked: watermarkedImageWeb.get_watermarked().slice(),
  };
};

export const fileToBase64 = (file: File) =>
  new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(('' + reader.result).replace(mimeRegex, ''));
    reader.onerror = error => reject(error);
  });

export const base64ToImage = (str: string) =>
  toWatermarkedImage(watermark_image(base64ToUInt8Array(str)));
