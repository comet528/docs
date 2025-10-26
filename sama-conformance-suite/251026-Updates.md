# 26 Oct 25 - sama conformance suite update goals

Today I would like to focus on 4 areas
- speeding testing via direct file injection
- updating the vue framework so it can use node 22
- ui testing via playwright framework
- preparation for next major piece of work that involves
    - updating the swagger for ksa
    - understanding how the tst case work
      - ensuring that all the tests work appropriately
      - suggesting ways to improve the test framework, test definitions etc
    - adding new tests to cover the new swagger
      - likely understanding better the business rules associated with the swagger
        - we need to figure out whats to enable this as the business rules are stored in private confluence pages
    get the schema validatator working
        - as part of the testcases for responses - ensuring adherence to the schema
        - as part of the test case checking for requests - i.e. a request should be allowed to break the schema but it should be flagged as doing such so that only testcases intended to break the request scheam actually do.

Go away and give me a proposal on how you deal with each of the above areas and the comparative amount of work required for each so that I can understand the size of effort for each task.
Keep the discussion at a relatively high level - which I decide to start working on each of the 4 we can deep dive implemenation of each at that point.




