export default function FrameCard(props: React.PropsWithChildren<{ name?: string, className?: string }>) {
  return (
    <div className={`flex flex-col mx-auto min-h-[60px] ${props.className || ''}`}>
      {props.children}
      <div className='text-center font-bold text-2xl pb-2 order-first'>{props.name}</div>
    </div>
  )
}