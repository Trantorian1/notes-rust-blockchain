#[cfg(all(test, feature = "solutions"))]
mod solutions {
    use crate::{
        question_0, question_1, question_10, question_11, question_2, question_3, question_4,
        question_5, question_6, question_7, question_8, question_9, tests::sanity_check,
    };

    fn incorrect(msg: &str) {
        log::error!("INCORRECT] {msg}");
        panic!();
    }

    fn correct(msg: &str) {
        log::info!("CORRECT] {msg}")
    }

    #[test_log::test]
    fn solution_0() {
        log::info!("[ QUESTION 0 ]");
        sanity_check(question_0());
        match question_0() {
            'a' => incorrect("While Blockchain technologies CAN be used to create digital currencies, they also have many more applications, and therefore purposes."),
            'b' => incorrect("While Blockchain technologies CAN in theory replace banking system, they have other purposes."),
            'c' => correct("Blockchain technologies allow use mathematics to prove the validity of new information (or transitions) in our shared narrative."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_1() {
        log::info!("[ QUESTION 1 ]");
        sanity_check(question_1());
        match question_1() {
            'a' => incorrect("It is possible for a game to end even if the players do not agree on its rules."),
            'b' => correct(""),
            'c' => incorrect("'Consensus' describes a state of agreement in a game, not a player's ability to convince others."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_2() {
        log::info!("[ QUESTION 2 ]");
        sanity_check(question_2());
        match question_2() {
            'a' => incorrect("This is TRUE: web 2.0 is inherently centralized since you need to trust the service provider."),
            'b' => incorrect("This is TRUE: it is necessary to trust service providers since there is no mathematical way of verifying their actions are valid."),
            'c' => correct("This is FALSE: while it is possible to ADD mathematical security to Web 2.0, it is not inherent."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_3() {
        log::info!("[ QUESTION 3 ]");
        sanity_check(question_3());
        match question_3() {
            'a' => incorrect(""), 
            'b' => correct("Keep in mind that while this is a property blockchain CAN have it is still possible for our narrative to be changed through chain splits. Remember: blockchain is before all a social construct, as was the case during the DAO hack."),
            'c' => incorrect("Once information is stored on a Blockchain it can always be accessed as long as it is part of our shared narrative. Issues like chain splits and finality might affect that narrative though, as we will be seeing in more depth."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_4() {
        log::info!("[ QUESTION 4 ]");
        sanity_check(question_4());
        match question_4() {
            'a' => correct("Thanks to the mathematical guarantees of Blockchain, it is possible to construct system with no source of centralized control."),
            'b' | 'c' => incorrect("While decentralized governance is possible in Blockchains, it is not necessary, with most Blockchains not having any (such as in the case of Ethereum and Bitcoin)."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_5() {
        log::info!("[ QUESTION 5 ]");
        sanity_check(question_5());
        match question_5() {
            'a' => incorrect("While Blockchain is still very technically limited, it struggles more in other aspects."),
            'b' => incorrect("Assuming the general population has an impact on the politics ob its country, then government regulations happen in part as a result of public opinion. Therefore government regulation are partly consequential to the general population, and this is not the correct answer."),
            'c' => correct("Assuming the general population has an impact on the politics of its country, then government regulations happen in part as a result of public opinion. Therefore government regulation are partly consequential to the general population. Moreover, technology is only as useful as people understand it to be, even if it is allowed to be used."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_6() {
        log::info!("[ QUESTION 6 ]");
        sanity_check(question_6());
        match question_6() {
            'a' => correct("Allowing anyone to inspect the code mitigates the risk of code exploits by allowing for both better accountability in case of mistakes, as well as a much larger pool of verifiers."),
            'b' => incorrect("While auditing code might make it safer locally, it does not help with accountability as different code might still be uploaded to the Blockchain than the one audited."),
            'c' => incorrect("This raises again the issue of centralization: how can we guarantee a set of rules is followed in the development of Blockchain without resorting to a central authority? This is a cyclical problem."),
            _ => unreachable!(),
        }
    }

    #[test_log::test]
    fn solution_7() {
        log::info!("[ QUESTION 7 ]");
        sanity_check(question_7());
        match question_7() {
            'a' | 'b' => incorrect("This is an example of vulnerability in trusting a centralized entity, since the code itself was not publicly available."),
            'c' => correct(""),
            _ => incorrect("")
        }
    }

    #[test_log::test]
    fn solution_8() {
        log::info!("[ QUESTION 8 ]");
        sanity_check(question_8());
        match question_8() {
            'a' => incorrect("As we will see later in this book, it doesn't make any sense to process chains in parallel as they each represent a separate, competing version of a shared narrative."),
            'b' => correct("This was the case during the creation of Ethereum Classic after the DAO attack."),
            'c' => incorrect("A chain split refers to a split in the shared narrative of a Blockchain."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_9() {
        log::info!("[ QUESTION 9 ]");
        sanity_check(question_9());
        match question_9() {
            'a' => incorrect("Cryptographic guarantees can make Web 3.0 immune to a certain class of forgeries/impersonation attempts, but they do not protect from purely social problems such as the manipulation tactics used in social engineering."),
            'b' => correct("This is a result of the current crypto-illiteracy of many Web 3.0 users and the population at large."),
            'c' => incorrect("Web 3.0 is AT LEAST as vulnerable to social engineering as Web 2.0, however the current lack of technical awareness around the field makes it MORE vulnerable."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_10() {
        log::info!("[ QUESTION 10 ]");
        sanity_check(question_10());
        match question_10() {
            'a' => incorrect("If a system is trustless then by definition it requires no trust."),
            'b' => incorrect("A system is MORE secure if it does not rely on trust as it then has no centralized point of failure."),
            'c' => correct("Blockchain technologies do not protect us from human error."),
            _ => unreachable!()
        }
    }

    #[test_log::test]
    fn solution_11() {
        log::info!("[ QUESTION 11 ]");
        sanity_check(question_11());
        match question_11() {
            'a' => correct("Allowing for Blockchains to adapt themselves to community changes and provide a central hub for discussion allows for greater resilience to issues of social turmoil."),
            'b' => incorrect("The entire point of Blockchain is to avoid centralization."),
            'c' => incorrect("The failure of Blockchains to adapt and provide a point of open discussion is exactly the reason why chain splits occur, such as in the case of the DAO attack and Ethereum Classic."),
            _ => unreachable!()
        }
    }
}
