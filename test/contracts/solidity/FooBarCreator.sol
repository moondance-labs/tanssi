// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

contract Foo {
  function newBar()
    public
    returns(Bar newContract)
  {
    Bar b = new Bar();
    return b;
  }
}

contract Bar {
  function getNumber()
    public
    pure 
    returns (uint32 number)
  {
    return 10;
  }    
}