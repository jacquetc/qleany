// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_interactor_export.h"

#include "brand/brand_dto.h"

#include <QObject>

namespace Simple::Interactor
{

using namespace Simple::Contracts::DTO::Brand;

class SIMPLE_EXAMPLE_INTERACTOR_EXPORT BrandSignals : public QObject
{
    Q_OBJECT
  public:
    explicit BrandSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(BrandDTO dto);
    void updated(BrandDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(BrandDTO dto);
    void getAllReplied(QList<BrandDTO> dtoList);
};
} // namespace Simple::Interactor