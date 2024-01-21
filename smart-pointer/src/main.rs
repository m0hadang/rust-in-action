/*
*
* 원시 포인터 : *mut T, *const T
* - 빠르다
* - 외부 세계와 상호 작용 가능
* - unsafe
*
* 스마트 포인터 : 원시 포인터 포다 크기 커짐
*
* - Box<T> : 힙에 저장
* - Rc<T> : 참조 카운트 포인터. 값에 대한 공유 접근 가능. 런타임 비용 발생. 쓰레드 안전하지 않음.
* - Arc<T> : 값에 대한 공유 접근 가능하며 쓰레드 안정성 보장. 런타임 비용 발생.
* - Cell<T> : 변경할 수 없는 값을 변경. 내부 가변성. 성능 문제 발생.
* - RefCell<T> : 불편 참조에 변경을 행할 수 있다. 내부 가변성, Rc, Arc 안에 중첩될 수 있으며 불변
* 참조만 허용. 런타임 비용. 컴파일 타임 타입 검증 누락
* - Cow<T> : 읽기 전용으로 쓰일 때는 쓰기 동작을 하지 않음.
* - String : 문자열
* - Vec<T> : 벡터
*
* - RawVec<T> : 메모리 할당을 직접 제어할 수 있는 Vec. unsafe.
* alloc::raw_vec::RawVec 타입을 바탕으로 Vec<T>와 VecDeque<T>가 만들어진다.
*
* - Unique<T> : String과 같은 타입의 기반이며 값을 독점적으로 소유.
* - Shared<T> : 공유된 소유권. 비어 있더라도 T의 크기만큼 메모리를 정렬할 수 있음.
* Unique는 String, Box<T>같은 타입과 포인터 필드 Vec<T>의 기반이 되는 스마트 포인터다
* Shared는 Rc<T>, Arc<T>의 기반이고 공유 접근이 필요한 상황을 다룰수 있다
* Unique, Shared는 다른 포인터를 만들떄 사용하는 기반 타입이며 이 기반 타입으로 String, Rc 같은
* 다른 스마트 포인터를 제조할 수 있다
* 그러나 어플리케이션에서 Unique, Shared를 그대로 사용 하는 것은 지양해야 한다.
*
* - std::rc::Weak : 참조 카운트 증가 없이 참조 가능. 쓰레드 안전하지 않음.
* - std::arc::Weak : 참조 카운트 증가 없이 참조 가능. 쓰레드 안전성 보장.
* Weak 는 포인터의 순환을 방지 할 수 있다
*
* - std::vecll::UnsafeCell : 자신이 만든 타입에 내부 가변성을 적용
* UnsafeCell 기반으로 Cell<T>와 RefCell<T> 가 만들어 진다
*
* */
fn is_strong1(password: String) -> bool {
    password.len() > 5
}
fn is_strong2<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}
fn is_strong3<T: Into<String>>(password: T) -> bool {
    // casting
    password.into().len() > 5
}

fn main() {
    is_strong1(String::from("pwd"));

    is_strong2(String::from("pwd"));
    is_strong2("pwd");

    is_strong3(String::from("pwd"));
    is_strong3("pwd");
}
