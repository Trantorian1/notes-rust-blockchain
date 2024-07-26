mod solution;

const A: char = 'a';
const B: char = 'b';
const C: char = 'c';

#[allow(dead_code)]
// ANCHOR: q0
// TODO:
// > What is the main purpose of a Blockchain technologies?
// >
// > - a) To create digital currencies.
// > - b) To replace traditional banking systems.
// > - c) To achieve mathematical consensus in a narrative.
// >
pub fn question_0() -> char {
    todo!()
}
// ANCHOR_END: q0

#[allow(dead_code)]
// ANCHOR: q1
// TODO:
// > In the context of the high school marble problem, what does "consensus" mean?
// >
// > - a) When players finish the game.
// > - b) When multiple parties come to an agreement on the state of the game.
// > - c) When one player convinces others to give up their marbles.
// >
pub fn question_1() -> char {
    todo!()
}
// ANCHOR_END: q1

#[allow(dead_code)]
// ANCHOR: q2
// TODO:
// > Which of the following is NOT true about Web 2.0 services?
// >
// > - a) Web 2.0 is centralized.
// > - b) Web 2.0 requires trust in service providers
// > - c) Web 2.0 is inherently cryptographically secure.
// >
pub fn question_2() -> char {
    todo!()
}
// ANCHOR_END: q2

#[allow(dead_code)]
// ANCHOR: q3
// TODO:
// > What does it mean for Blockchain to be a "continuation of Humanity's history of
// > shared storytelling"?
// >
// > - a) Blockchain is primarily used for writing fiction.
// > - b) Blockchain allows for a fixed, unchangeable narrative over time.
// > - c) Blockchain is vulnerable to degradation like oral traditions.
// >
pub fn question_3() -> char {
    todo!()
}
// ANCHOR_END: q3

#[allow(dead_code)]
// ANCHOR: q4
// TODO:
// > Why is Web 3.0 arguably more democratic than Web 2.0?
// >
// > - a) It doesn't require trust in centralized entities.
// > - b) It allows for direct voting on all decisions.
// > - c) It's controlled by elected representatives.
// >
pub fn question_4() -> char {
    todo!()
}
// ANCHOR_END: q4

#[allow(dead_code)]
// ANCHOR: q5
// TODO:
// > What is described as potentially the greatest hurdle for Web 3.0 to overcome?
// >
// > - a) Technical limitations of blockchain technology.
// > - b) Government regulations.
// > - c) Crypto-literacy of the general population.
// >
pub fn question_5() -> char {
    todo!()
}
// ANCHOR_END: q5

#[allow(dead_code)]
// ANCHOR: q6
// TODO:
// > What are some ways to mitigate the risk of code exploits in blockchain
// > applications?
// >
// > - a) By allowing anyone to inspect the code.
// > - b) By using only professionally audited code.
// > - c) By implementing strict regulations on developers.
// >
pub fn question_6() -> char {
    todo!()
}
// ANCHOR_END: q6

#[allow(dead_code)]
// ANCHOR: q7
// TODO:
// > What event is an example of vulnerabilities in trusting code?
// >
// > - a) The Equifax data breach.
// > - b) The Cambridge Analytica scandal.
// > - c) The DAO attack.
// >
pub fn question_7() -> char {
    todo!()
}
// ANCHOR_END: q7

#[allow(dead_code)]
// ANCHOR: q8
// TODO:
// > In the context of Blockchain, what is a "chain split"?
// >
// > - a) A method for increasing transaction speed by processing parallel chains.
// > - b) A division of the community resulting in two competing versions
// >      of the blockchain.S
// > - c) A way to distribute mining rewards more fairly.
// >
pub fn question_8() -> char {
    todo!()
}
// ANCHOR_END: q8

#[allow(dead_code)]
// ANCHOR: q9
// TODO:
// > What is the relationship between Web 3.0's cryptographic guarantees and its
// > vulnerability to social engineering?
// >
// > - a) Cryptographic guarantees make Web 3.0 immune to social engineering.
// > - b) Web 3.0 may be more vulnerable to social engineering due to its complexity.
// > - c) Web 3.0 is equally as vulnerable to social engineering as Web 2.0.
// >
pub fn question_9() -> char {
    todo!()
}
// ANCHOR_END: q9

#[allow(dead_code)]
// ANCHOR: q10
// TODO:
// > What paradox can you highlight regarding the trustless nature of Web 3.0?
// >
// > - a) Trustless systems require more trust from users.
// > - b) The more trustless a system becomes, the less secure it is.
// > - c) While transactions are trustless, users still need to trust the developers
// >      who wrote the code.
// >
pub fn question_10() -> char {
    todo!()
}
// ANCHOR_END: q10

#[allow(dead_code)]
// ANCHOR: q11
// TODO:
// > Which is a way to address the issue of diverging opinions in blockchain
// > communities?
// >
// > - a) Exploring decentralized governance structures inspired by real-world
// >      solutions.
// > - b) Implementing centralized control mechanisms.
// > - c) Creating immutable rules that can never be changed
// >
pub fn question_11() -> char {
    todo!()
}
// ANCHOR_END: q11

#[cfg(test)]
mod tests {
    use super::*;

    pub fn sanity_check(c: char) {
        if c != A && c != B && c != C {
            log::error!("Answer to a question can only be 'a', 'b', or 'c'");
            panic!();
        }
    }

    #[test]
    fn sanity_check_0() {
        sanity_check(question_0());
    }

    #[test]
    fn sanity_check_1() {
        sanity_check(question_1());
    }

    #[test]
    fn sanity_check_2() {
        sanity_check(question_2());
    }

    #[test]
    fn sanity_check_3() {
        sanity_check(question_3());
    }

    #[test]
    fn sanity_check_4() {
        sanity_check(question_4());
    }

    #[test]
    fn sanity_check_5() {
        sanity_check(question_5());
    }

    #[test]
    fn sanity_check_6() {
        sanity_check(question_6());
    }

    #[test]
    fn sanity_check_7() {
        sanity_check(question_7());
    }

    #[test]
    fn sanity_check_8() {
        sanity_check(question_8());
    }

    #[test]
    fn sanity_check_9() {
        sanity_check(question_9());
    }

    #[test]
    fn sanity_check_10() {
        sanity_check(question_10());
    }

    #[test]
    fn sanity_check_11() {
        sanity_check(question_11());
    }
}
