#pragma once

#include "car.h"
#include "car/car_dto.h"
#include "car/car_with_details_dto.h"

using namespace Simple;
using namespace Simple::Contracts::DTO::Car;

namespace Simple::Application::Features::Car
{

template <class T> T map(const Domain::Car &source)
{
    T carDto;
    carDto.setId(source.id());

    return carDto;
}

template <> CarDTO map<CarDTO>(const Domain::Car &source)
{
    CarDTO carDto;
    carDto.setId(source.id());

    //    if(source.metaData().isPassengersSet() || source.metaData().isPassengersLoaded()){
    //        carDto.setPassengers(source.passengers());
    //    }

    return carDto;
}

} // namespace Simple::Application::Features::Car
