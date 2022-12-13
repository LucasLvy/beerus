Feature: Beerus core feature

    Scenario: If we have normal conditions we query the starknet state root
        Given normal conditions
        Given starknet state root is 0x01
        When I query starknet state root
        Then I get 0x01
