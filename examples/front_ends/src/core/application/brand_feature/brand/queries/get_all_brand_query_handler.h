// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "front_ends_example_application_brand_export.h"

#include "repository/interface_brand_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Application::Features::Brand::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_BRAND_EXPORT GetAllBrandQueryHandler : public QObject
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

} // namespace FrontEnds::Application::Features::Brand::Queries