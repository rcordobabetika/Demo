Feature:
  Verify different GET operations using REST-assured


  Scenario: Author of the post verification
    Given  I perform GET operation for "/songs"
    And I perform GET for the post number ""
    Then I should see the author name as "Yolanda"

  Scenario: Collection of authors in the post
    Given  I perform GET operation for "/songs"
    Then I should see the author name as

  Scenario: Parameter of GET
    Given  I perform GET operation for "/songs"
    Then I should see the author name as
