import React, { Suspense } from 'react';
import './App.css';

const Content = React.lazy(() => import('./Content/Content'));

export default () => (
  <Suspense fallback={<div>Loading...</div>}>
    <Content />
  </Suspense>
);
