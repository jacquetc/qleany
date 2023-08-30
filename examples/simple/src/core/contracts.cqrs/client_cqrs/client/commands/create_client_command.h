// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "client/create_client_dto.h"


namespace Simple::Contracts::CQRS::Client::Commands
{
class CreateClientCommand
{
  public:
    CreateClientCommand()
    {
    }


    Simple::Contracts::DTO::Client::CreateClientDTO req;

};
} // namespace Simple::Contracts::CQRS::Client::Commands