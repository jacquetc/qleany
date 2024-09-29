// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/update_car_dto.h"

namespace Simple::Contracts::CQRS::Car::Commands
{
class UpdateCarCommand
{
  public:
    UpdateCarCommand()
    {
    }

    Simple::Contracts::DTO::Car::UpdateCarDTO req;
};
} // namespace Simple::Contracts::CQRS::Car::Commands