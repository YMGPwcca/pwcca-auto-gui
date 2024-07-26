import style from '../styles/Comps.module.css'

export default function FrameCard(props: React.PropsWithChildren<{ name?: string, className?: string }>) {
  return (
    <div className={`${style.card} ${props.className || ''}`}>
      {props.children}
      <div className={style.text}>{props.name}</div>
    </div>
  )
}