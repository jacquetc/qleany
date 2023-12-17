// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "brand/brand_dto.h"

#include <QObject>

namespace Simple::Controller
{

using namespace Simple::Contracts::DTO::Brand;

class SIMPLEEXAMPLE_CONTROLLER_EXPORT BrandSignals : public QObject
{
    Q_OBJECT
  public:
    explicit BrandSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void getReplied(BrandDTO dto);
    void getAllReplied(QList<BrandDTO> dtoList);
    void created(BrandDTO dto);
    void updated(Contracts::DTO::Brand::BrandDTO dto);
    void allRelationsInvalidated(int id);
};
} // namespace Simple::Controller