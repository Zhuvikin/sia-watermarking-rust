import {
  get_watermarking_process,
  ImageModel as ImageModelWasm,
  WatermarkingProcess as WatermarkingProcessWasm,
} from 'react-image';
import { ImageModel, WatermarkingProcess } from './image';

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

export const toImageModel = (imageModelWasm: ImageModelWasm): ImageModel => {
  return {
    width: imageModelWasm.get_width(),
    height: imageModelWasm.get_height(),
    data: imageModelWasm.get_data().slice(),
  };
};

export const toWatermarkingProcess = (
  watermarkingProcessWasm: WatermarkingProcessWasm,
): WatermarkingProcess => {
  return {
    sourceFormat: watermarkingProcessWasm.get_source_format(),
    source: toImageModel(watermarkingProcessWasm.get_source()),
    watermarked: toImageModel(watermarkingProcessWasm.get_watermarked()),
  };
};

export const fileToBase64 = (file: File) =>
  new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => resolve(('' + reader.result).replace(mimeRegex, ''));
    reader.onerror = error => reject(error);
  });

export const sourceBase64ToWatermarkingProcess = (
  str: string,
  embeddingDepth: number,
  featuresQuantizationStep: number,
) =>
  toWatermarkingProcess(
    get_watermarking_process(
      base64ToUInt8Array(str),
      embeddingDepth,
      featuresQuantizationStep,
    ),
  );
