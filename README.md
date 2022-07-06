# WriteLog

Well, not weBLOG, because blog.near and blog.testnet is reserved during Genesis. 

Anyways, you could visit the current contract on wlog.near.page. There are indeed quite a lot 
of stuffs not completed due to time constraints; but we try to make some functionality works 
and frontend as user-friendly (though not necessarily the most beautiful) as possible. 

Basically, a basic wlogging website to write whatever. It's not everything supported; but at 
least you could share words and images (memes!); one haven't tried videos support (nor planning
to support videos) though. It doesn't meet the aim of "writing log": that someone could do on 
"vlog.near.page" or something. 

And for more design reasonings, please visit [the about page](https://wlog.near.page/about). One 
can't say one included everything there; but at least some of the stuffs that one thought and 
have the chance to write it down; it'll be there. 

As for how to use the website, we have [another page here]() which you could check out. One supposes
one test that it support international language (like Chinese); so the "how to" might need 
translation into multiple pages as well. 

Anyway, to signal me, you could open an issue, leave a chat in the GitHub discussion box, or whatever that
GitHub notify me to pull my attention on this page. Voilà; have fun! 

---

Additional stuffs that could further develop in the future: 
- Different colors (like MIrror.xyz) for user choice; with a default of Bootstrap color. 
- Dark mode. 
- Image support (with or without cover image; thumbnail at the very least).
- Thumbs up (a.k.a "clap" on Mirror). 
- Some not-security-but-annoying issues with the smart contract (really, although one solves the 
security issues; it doesn't mean it cannot be annoying.)
- cross-website saving: currently non-IPFS save is using `localStorage`; but you could save it
somewhere else; either the blockchain on-chain (so it could be deleted when carved to stone on IPFS), 
or you could save to a database; whatever you want to do it. 
- [On-chain search engine](https://mirror.xyz/0xa32e05D545FEc9cADb46BEB0839E3Ac0A9E39d9B/4N_KpWccWPNW7TNHqkmYSHIcvAO6svd9erpCEvLlAkI)


The aim is for simplicity. Not feature-less; not lightweight version. It could have lots of features, but
it needs to be simple. It should be so simple that a contract audit is not required to find all the security
bugs that might arise from the contract. Straightforward, short, simple. 

---

Test building a web4 example on testnet. 

Because one haven't learn how workspaces-rs works, we haven't write any integration tests on this. As for unit tests; one prefer to write integration tests than unit test: it's more clear what's going on interacting from the user's side; unless we need data only accessible within the contract. 

