import { useState } from "react";

export default function ChainButton({
  img,
  children,
  disconnect,
}: {
  img: string;
  disconnect: () => any;
  children: JSX.Element | string | string[];
}) {
  const [hoverImg, setHoverImg] = useState(img);

  return (
    <div
      className="chain-button"
      onMouseEnter={() => setHoverImg("/disconnect.svg")}
      onMouseLeave={() => setHoverImg(img)}
      onClick={() => disconnect()}
    >
      <div>
        <img src={hoverImg} alt="icp" />
        {children}
      </div>
    </div>
  );
}
