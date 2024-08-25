export default function FrameCard(props: React.PropsWithChildren<{ name?: string }>) {
  return (
    <div id={props.name} className={`flex flex-col`}>
      <div className='font-bold text-2xl mb-2 text-center'>{props.name}</div>
      {props.children}
    </div>
  )
}