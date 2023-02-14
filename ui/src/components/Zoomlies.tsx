import Image from "next/image";

const HERO = "/blue-hero.svg";

export default function Zoomlie() {
  return <Image height={300} width={150} src={HERO} alt="Hero" />;
}
