import React, { useEffect, useState } from 'react';
import WatermarkingPreview from '../WatermarkingPreview/WatermarkingPreview';
import init from 'react-image';
import * as loadImage from './peppers.jpg';
import { base64ToImage, fileToBase64, toDataURL } from './utils';
import { WatermarkedImage } from './image';

type ContentProps = {};

const Content = (_: ContentProps) => {
  const [image, setImage]: [
    WatermarkedImage | undefined,
    React.Dispatch<React.SetStateAction<WatermarkedImage | undefined>>,
  ] = useState();

  useEffect(() => {
    init().then(async () => {
      const imageDataUrl = await toDataURL(loadImage.default);
      setImage(base64ToImage(imageDataUrl));
    });
  }, []);

  const loadImageFromFile = (file: File) => {
    fileToBase64(file).then(base64EncodedImage => {
      setImage(base64ToImage(base64EncodedImage));
    });
  };

  if (!image) {
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
      <WatermarkingPreview image={image} />
    </div>
  );
};

export default Content;
