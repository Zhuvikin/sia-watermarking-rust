export type WatermarkingProcess = {
  sourceFormat: string;
  source: ImageModel;
  watermarked: ImageModel;
};

export type ImageModel = {
  width: number;
  height: number;
  data: Uint8Array;
};