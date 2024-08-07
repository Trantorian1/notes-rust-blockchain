# The Ugly: the pitfalls of Web 3.0

As discussed previously, Web 3.0 is _not_ a silver bullet. While it has many interesting properties that contribute to it being _theoretically_ more secure than Web 2.0, these mathematical security guarantees are by themselves not enough -as we will see here.

## Social Vulnerabilities

While Web 3.0 provides safer guarantees in terms of integrity and authenticity of information, it is arguably _more_ vulnerable in it current state to the main type of cyberattack technique used in Web 2.0: **social engineering**.

> **social engineering**: the manipulation of people into taking actions that compromise their security or privacy.
>
> ex: _a common type of social engineering is_ phishing, _where an attacker convinces it's target to click on a malicious link which has unintended consequences such as installing a virus on the target computer or, in the case of cryptocurrencies, draining all the victim's funds._

The reason why Web 3.0 is currently more vulnerable to this type of attack is because of the complexity of the subject at hand: Web 3.0 is fundamentally multidisciplinary, involving advanced topics from economics, cryptography, game theory and other fields still. With most of the world population still crypto-illiterate, this is an easy point of attack for malicious actors, as can be seen in the many scams, ponzis and such which plague the crypto world today.

It can be argued that crypto-literacy is at large the greatest hurdle yet to be overcome by Web 3.0.

## Trust is still the Name of the Game

Web 3.0's greatest strength is it's ability to provide trustless ways to execute code (we will examine those more in depth over the course of this book). But what about the code itself? It is important to realize that here again we need to *trust* the developers who wrote the code:

- _We need to trust_ the code is bug-free, that is to say there is no _unknown_, _unintentional_ vulnerability.
- _We need to trust_ the code is exploit-free, that is to say there is no _hidden_, _intentional_ vulnerability.

> **The DAO attack** is one of the most notorious examples of the issues that can arise from blindly trusting code.
>
> _On June 17 2016, an unidentified hacker began attacking The DAO, the largest Ethereum DApp (decentralized application) at the time, holding a total of $250_ **million** _in assets. They used what is now known as a "re-entrency attack", a then unknown[^1], unintentional bug in The DAO's code. The attack resulted in one of the most troubled times in Ethereum's history and the creation of Ethereum Classic._

As mentioned before, Blockchain technologies do not protect us from logical errors, and so we are still vulnerable to code exploits such as these. Even worse: nothing stops a malevolent actor from intentionally creating bad code that can later be exploited.

This can be mitigated at the protocol level by allowing _anyone_ to inspect the code of Blockchain applications, but this does not guarantee that a bug or exploit won't be missed[^2].

## Inconsistencies in a Shared Narrative

At the core of Web 3.0 is the notion of shared narrative we discussed before. You can see a Blockchain as a community of people who believe in one such narrative. This is the case of networks like Ethereum or Bitcoin.

> Blockchains exist because people come together around the world in unison to interact together according to a common protocol, or set of rules.

What happens however when the rules to a common protocol are broken? This is typically the case during periods of intense community discourse. To continue on the example of The DAO, there was a strong divide in the Ethereum community at the time as to which was the best way to address the hack, or if it should be addressed at all.

Some members of the community believed the attacker had done nothing wrong: indeed, they had only been using the code as it was provided. It was a bit as if a bank had left their vaults out in the open and the bankers had started posting blog posts on how to open it. To punish the thief would go against the principle of immutability of Blockchains: it would be like taking our great shared story, finding a passage we didn't like, and changing it into something else entirely. Now a part of that story is gone forever.

This divide into two camps led to two version of the same Blockchain:

- **Ethereum**, were the attack had been rolled back. It was as if we had time traveled into the past to stop a bank robbery and that was now our new story.
- **Ethereum Classic**, where a portion of the community had decided to do _nothing_ in order to preserve what they saw as the core values of Blockchain.

It was as if all of a sudden your favorite character died in a series, and you and a _very large_ group of friends all decided to upkeep you own version of the plot where they kept living: now we have two contending story lines which are battling it out for which is the real one -yikes.

This is what is known as a **chain split**, and has happened in other chains as well (and again in Ethereum with _Ethereum POW_). 

> It is important to realize that despite all its cryptographic guarantees and advanced game theory, Web 3.0 and Blockchain remains before all a _social construct_, and can be disrupted in periods of social turmoil. This is good because it allows for resistance to unfair change by dissident groups, something that is not so easy in the real world, but at times it can seriously hamper the stability of these networks.

## What we can do better: solutions to these issues

What the previous issues highlight above all else is that as much as Web 3.0 and Blockchain are a mathematical problem, they are also a complex social problem involving many different people and diverging points of view.

Ethereum suffered because of this in its early days since it did not have any dedicated way to handle these differences in opinions at the protocol level. In a way, while Ethereum was the story, there was no central meeting point where storytellers could convene to discuss its future and potential amendments to it.

In the real world, we have structures to deal with these issues: governments. But how can we take ideas from existing, centralized structures of governance and implement them in a decentralized way? While we will be focusing on more technical details (Blockchain Governance will be a topic for a future book), keep this idea in the back of your mind as a potential solution.

_With this brief introduction to the advantages, drawbacks and challenges of Blockchain, let us now dive into the core subject at hand._

---

[^1]: In reality, the exploit had already been brought to the light by various members of the community, with Christian ReitwieBner being the first to discover it and warn the developers behind The DAO, while Peter Vessenes was the first to write a blog post about it less than two weeks before the attack. The bug was deemed of little importance however and even Vitalik at the time tweeted he was not worried by the news and was still buying DAO tokens.

[^2]: For a long time, it was believed that open-source software -that is to say software where anyone can view and contribute to the code- was impervious to bad actors since anyone could verify the code and point out any malintent or error. Recent exploits such as the notorious [xz-utils backdoor](https://arstechnica.com/security/2024/03/backdoor-found-in-widely-used-linux-utility-breaks-encrypted-ssh-connections/) have shown however that the complexity of certain projects has grown to the point where it is possible for malicious code to be (very) cleverly concealed in plain sight.
