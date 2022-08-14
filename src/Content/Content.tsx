import React, { useEffect, useState } from 'react';
import WatermarkingPreview from '../WatermarkingPreview/WatermarkingPreview';
import init from 'react-image';
import * as loadImage from './peppers.jpg';
import {
  sourceBase64ToWatermarkingProcess,
  fileToBase64,
  toDataURL,
} from './utils';
import { WatermarkingProcess } from './image';

const EMBEDDING_DEPTH = 500.0;
const FEATURES_QUANTIZATION_STEP = 1.0;

type ContentProps = {};

function getWatermarkingProcess(sourceImageDataUrl: string) {
  return sourceBase64ToWatermarkingProcess(
    sourceImageDataUrl,
    EMBEDDING_DEPTH,
    FEATURES_QUANTIZATION_STEP,
  );
}

const Content = (_: ContentProps) => {
  const [watermarkingProcess, setWatermarkingProcess]: [
    WatermarkingProcess | undefined,
    React.Dispatch<React.SetStateAction<WatermarkingProcess | undefined>>,
  ] = useState();

  useEffect(() => {
    init().then(async () => {
      const sourceImageDataUrl = await toDataURL(loadImage.default);
      setWatermarkingProcess(getWatermarkingProcess(sourceImageDataUrl));
    });
  }, []);

  const loadImageFromFile = (file: File) => {
    fileToBase64(file).then(base64EncodedImage => {
      setWatermarkingProcess(getWatermarkingProcess(base64EncodedImage));
    });
  };

  if (!watermarkingProcess) {
    return <div className="WatermarkingPreview">Loading...</div>;
  }

  return (
    <div>
      <div className="file-selector">
        <input
          type="file"
          onChange={e => loadImageFromFile(e.target.files![0])}
        />
      </div>
      <WatermarkingPreview watermarkingProcess={watermarkingProcess} />
    </div>
  );
};

export default Content;
