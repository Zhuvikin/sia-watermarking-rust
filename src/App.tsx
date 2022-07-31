import React, { Suspense } from 'react';
import './App.css';

const Content = React.lazy(() => import('./Content/Content'));

const App = () => (
  <Suspense fallback={<div>Loading...</div>}>
    <Content />
  </Suspense>
);

export default App;
