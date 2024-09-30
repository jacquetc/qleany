// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car/update_car_dto.h"

namespace FrontEnds::Contracts::CQRS::Car::Commands
{
class UpdateCarCommand
{
public:
    UpdateCarCommand()
    {
    }

    FrontEnds::Contracts::DTO::Car::UpdateCarDTO req;
};
} // namespace FrontEnds::Contracts::CQRS::Car::Commands