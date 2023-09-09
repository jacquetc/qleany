#pragma once

#include "car/car_dto.h"
#include "car/car_with_details_dto.h"
#include "controller_export.h"
#include <QObject>

namespace Simple::Controller
{

using namespace Simple::Contracts::DTO::Car;

class SIMPLEEXAMPLE_CONTROLLER_EXPORT CarSignals : public QObject
{
    Q_OBJECT
  public:
    explicit CarSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void getReplied(CarDTO dto);
    void getWithDetailsReplied(CarWithDetailsDTO dto);
    void getAllReplied(QList<CarDTO> dtoList);
    void created(CarDTO dto);
    void updated(Contracts::DTO::Car::CarDTO dto);
};
} // namespace Simple::Controller
