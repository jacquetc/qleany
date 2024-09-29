// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/queries/get_brand_query.h"
#include "front_ends_example_application_brand_export.h"

#include "repository/interface_brand_repository.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Brand::Queries;

namespace FrontEnds::Application::Features::Brand::Queries
{
class FRONT_ENDS_EXAMPLE_APPLICATION_BRAND_EXPORT GetBrandQueryHandler : public QObject
{
    Q_OBJECT
public:
    GetBrandQueryHandler(InterfaceBrandRepository *repository);
    Result<BrandDTO> handle(QPromise<Result<void>> &progressPromise, const GetBrandQuery &query);

private:
    InterfaceBrandRepository *m_repository;
    Result<BrandDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetBrandQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Brand::Queries