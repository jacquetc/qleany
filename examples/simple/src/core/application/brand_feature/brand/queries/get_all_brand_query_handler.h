// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "simple_example_application_brand_export.h"

#include "repository/interface_brand_repository.h"
#include <QPromise>

using namespace Simple;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;

namespace Simple::Application::Features::Brand::Queries
{
class SIMPLE_EXAMPLE_APPLICATION_BRAND_EXPORT GetAllBrandQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllBrandQueryHandler(InterfaceBrandRepository *repository);
    Result<QList<BrandDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceBrandRepository *m_repository;
    Result<QList<BrandDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Brand::Queries