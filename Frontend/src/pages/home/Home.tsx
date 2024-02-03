import { GreenButton } from './GreenButton';
import { RedButton } from './RedButton';
import { ReadState } from './ReadState';
import { YellowButton } from './YellowButton';

function Home() {
  return (
    <div>
      <GreenButton />
      <RedButton />
      <ReadState />
      <YellowButton />
    </div>
  );
}

export { Home };
