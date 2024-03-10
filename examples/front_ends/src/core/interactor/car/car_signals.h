// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_interactor_export.h"

#include "car/car_with_details_dto.h"

#include "car/car_dto.h"

#include "car/car_relation_dto.h"

#include <QObject>

namespace FrontEnds::Interactor
{

using namespace FrontEnds::Contracts::DTO::Car;

class FRONT_ENDS_EXAMPLE_INTERACTOR_EXPORT CarSignals : public QObject
{
    Q_OBJECT
public:
    explicit CarSignals(QObject *parent = nullptr)
        : QObject{parent}
    {
    }

Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(CarDTO dto);
    void updated(CarDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(CarDTO dto);
    void getWithDetailsReplied(CarWithDetailsDTO dto);
    void getAllReplied(QList<CarDTO> dtoList);

    void relationInserted(CarRelationDTO dto);
    void relationRemoved(CarRelationDTO dto);
};
} // namespace FrontEnds::Interactor