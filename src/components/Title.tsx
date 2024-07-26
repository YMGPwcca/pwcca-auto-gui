export default function Title() {
  return (
    <div className='bg text-center pt-2 pb-3 mobile:fixed mobile:top-8 mobile:self-center mobile:w-screen border-x-2 border-gray-600'>
      <span className='font-bold text-4xl mobile:text-3xl bg-gradient-to-r from-[rgb(135,17,193)] to-[rgb(36,114,252)] bg-clip-text text-transparent'>
        PwccaAuto
      </span>
      <span className='font-bold text-xs bg-gradient-to-r from-[rgb(36,114,252)] to-[rgb(135,17,193)] bg-clip-text text-transparent ml-1'>
        by Pwcca
      </span>
    </div>
  )
}