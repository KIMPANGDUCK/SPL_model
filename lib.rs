use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
//solana_program 이라는 모듈에서
/*
    account_info::AccountInfo
    entrypoint
    entrypoint::ProgramResult
    msg
    pubkey::Pubkey
    를 스코프로 불러온다.
*/

entrypoint!(process_instruction); // 엔트리포인트 매크로를 호출해야 컨트랙트가 시작되는 지점임을 안다.
                                  // 엔트리포인트는 프로그램을(솔라나 컨트랙트) 호출하는 유일한 방법임, solana 는 stateless 한 (무상태) 프로토콜이라 fn이 존재하 면 안된다.

// BPF 로더로 전달하기 때문에 프로그램을 build 할 때 cargo build 가 아닌 cargo build-bpf 를 사용한다.
//프로그램이 호출되면 BPF loader로 전달됨. 이 로더에서 프로그램을 다룸
// 서로 다른 BPF loader는 서로 다른 엔트리 포인트를 요구한다.
// mapping( entrypoint => BPF loader) entry_loader; 라고 생각하면 될듯?
// 여러개의 BPF loader가 존재해야하는 이유는 BPF 로더 자체도 프로그램이기 때문임.
//우리가 사용하는 BFP 로더는 3개의 인자를 가지는 함수 process_instruction 엔트리 포인트를 요구한다.
// 그래서 entrypoint! 매크로를 호출해서 process_instruction 을 인자로 전달하는 것이다.

fn process_instruction(
    program_id: &Pubkey, // simply id of this program pid도 Pubkey 임을 명심하라, 솔라나는 모두 account 단위로 구성이 되어있음  0
    accounts: &[AccountInfo], // 프로그램 모델 가서 더 알아보기
    instruction_data: &[u8], // data passed to the program by the caller, could be anything
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    ); // 로그메세지, process_instruction: program_id: accounts.len() accounts, data= instruction_data
    Ok(()) // ProgramResult -> Rust의 Result 타입을 Program에 관련하여 인터페이스 화 한것. 자세한 내용은 솔라나 프로그램 모델  참고
}

// 솔라나는 stateless 하다. 그래서 state를 저장하고 싶으면 accounts를 써야함
// 프로그램은 excuteable 이라는 딱지가 붙은 account이다.
// 각 account는 데이터랑 SOL이 얼마 있는지 가질 수 있음
// 각 account는 owner가 있어서 onwer만이 솔라나를  출금하거나 데이터를 수정할 수 있음
// 입금은 근데 아무나한테서 받을 수 있음
// 예시 어카운트는 오너 필드가 System Program 으로 설정되어있음

//Accounts들은 programs에 의해서만 소유될 수 있다.
//그럼 내 Sol이 나한테 귀속된게 아닌가?
//맞는 말이지만 무서워하지마라 funds are safu!
//솔라나를 전송하는 일도 프로그램에 의해서 핸들링 된다. 정확히는 솔라나에서 pre-depolyed 한 System Program 에 의해서 핸들링 된다.
//실제론 프로그램또한 프로그램에 의해서 소유된다. 기억해라 프로그램들은 account에 저장되어 있고 excuteable account 는 BPF loader에 의해 소유된다.
// BPF loader에 의해 소유되지 않는 프로그램은 BPF loader 자기 자신이랑 System Program 이다.
// BPF loader에 의해 소유되지 않는 프로그램은 Native loader에 의해서 소유되고 특별한 이권을 가진다, 예를들면 메모리 할당하거나 account 가 excuteable 하다고 marking을 하거나 하는 일들을 할 수 있음

// 시스템 프로그램을 살펴보면 프로그램이 모두 SOl 밸런스를 가지고 있음을 볼 수 있음
// 해당하는 프라이빗 키에 의해 서명이 되었을때만 전송이 실행됨 Signer가 필요하다.

//Program이 가진 account에 대한 full autonomy(자율성)를 가진다.
// 이러한 autonomy를 제한하는 것은 컨트랙트를 짠 사람한테 달려있는 것
// 프로그램의사용자가 정말 그렇게 컨트랙트를 짠 사람이 제한했는지 확인 하는 것임
